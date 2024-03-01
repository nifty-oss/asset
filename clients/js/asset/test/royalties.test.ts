import { TokenStandard } from '@metaplex-foundation/mpl-token-metadata';
import {
  generateSigner,
  percentAmount,
  publicKey,
} from '@metaplex-foundation/umi';
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import {
  string,
  publicKey as publicKeySerializer,
} from '@metaplex-foundation/umi/serializers';
import { mplToolbox } from '@metaplex-foundation/mpl-toolbox';
import { PublicKey } from '@solana/web3.js';
import test from 'ava';
import {
  Vault,
  create,
  fetchVault,
  findVaultPda,
  niftyBridge,
  Discriminator as BridgeDiscriminator,
  State as BridgeState,
  BRIDGE_PROGRAM_ID,
  bridge,
  findBridgeAssetPda,
} from '@nifty-oss/bridge';
import {
  Account,
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  fetchAsset,
  getExtensionSerializerFromType,
  group,
  niftyAsset,
  not,
  pubkeyMatch,
  royalties,
  update,
} from '../src';
import { createNft, createProgrammableNft, createVerifiedNft } from './_setup';

const createUmi = async () =>
  (await basecreateUmi())
    .use(mplToolbox())
    .use(niftyBridge())
    .use(niftyAsset());

test('pubkeymatch failing blocks a transfer', async (t) => {
  const umi = await createUmi();
  const owner = generateSigner(umi);
  const notOwner = generateSigner(umi);

  const basisPoints = BigInt(550);

  // And a Token Metadata programmable non-fungible.
  const mint = await createProgrammableNft(umi, {
    name: 'pNFT Bridge Asset',
    uri: 'https://asset.bridge',
    symbol: 'BA',
    sellerFeeBasisPoints: percentAmount(5.5),
    tokenOwner: owner.publicKey,
  });

  // When we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // And the asset is created.
  const asset = umi.eddsa.findPda(BRIDGE_PROGRAM_ID, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(mint.publicKey),
  ]);

  const pubkeyMatchConstraint = pubkeyMatch(Account.Asset, [
    publicKey(PublicKey.default),
  ]);
  const defaultConstraint = not(pubkeyMatchConstraint);

  t.like(await fetchAsset(umi, asset), <Asset>{
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://asset.bridge',
      },
      {
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: defaultConstraint,
      },
    ],
  });

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: BridgeDiscriminator.Vault,
    state: BridgeState.Idle,
    mint: mint.publicKey,
  });

  // We create a PubkeyMatch constraint that will block the transfer to the owner.
  const constraint = pubkeyMatch(Account.Recipient, [publicKey(notOwner)]);

  // We update the default royalties extension.
  const data = getExtensionSerializerFromType(
    ExtensionType.Royalties
  ).serialize(royalties({ basisPoints, constraint }));
  await update(umi, {
    asset: asset[0],
    payer: umi.identity,
    extension: {
      extensionType: ExtensionType.Royalties,
      length: data.length,
      data,
    },
  }).sendAndConfirm(umi);

  // Bridging the asset should fail.
  const promise = bridge(umi, {
    mint: mint.publicKey,
    owner,
    tokenStandard: TokenStandard.ProgrammableNonFungible,
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, {
    message: /Assertion Failure/,
  });

  // Now update the royalty extension to have the recipient be the pubkey match
  const newConstraint = pubkeyMatch(Account.Recipient, [publicKey(owner)]);

  // We update the default royalties extension.
  const newData = getExtensionSerializerFromType(
    ExtensionType.Royalties
  ).serialize(royalties({ basisPoints, constraint: newConstraint }));
  await update(umi, {
    asset: asset[0],
    payer: umi.identity,
    extension: {
      extensionType: ExtensionType.Royalties,
      length: data.length,
      data: newData,
    },
  }).sendAndConfirm(umi);

  // Bridging the asset should succeed.
  await bridge(umi, {
    mint: mint.publicKey,
    owner,
    tokenStandard: TokenStandard.ProgrammableNonFungible,
  }).sendAndConfirm(umi);

  // And we check that the asset was transferred.
  const transferredAsset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: mint.publicKey })
  );

  t.like(transferredAsset, <Asset>{
    holder: owner.publicKey,
  });
});

test.skip('pubkeymatch failing blocks a transfer on a group asset', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();
  const owner = generateSigner(umi);
  const notOwner = generateSigner(umi);

  const basisPoints = BigInt(550);

  // And a Token Metadata non-fungible representing a collection.
  const collectionMint = await createProgrammableNft(umi, {
    name: 'Bridge Collection',
    symbol: 'BA',
    uri: 'https://collection.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    isCollection: true,
    tokenOwner: owner.publicKey,
  });

  // And we create the collection asset on the bridge.
  await create(umi, {
    mint: collectionMint.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // And we create a Token Metadata non-fungible representing an asset
  // from the collection.
  const itemMint = await createVerifiedNft(umi, {
    name: 'Bridge Asset',
    symbol: 'BA',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    collectionMint: collectionMint.publicKey,
    tokenOwner: owner.publicKey,
  });

  // When we create the asset on the bridge.
  await create(umi, {
    mint: itemMint.publicKey,
    collection: findBridgeAssetPda(umi, { mint: collectionMint.publicKey }),
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // Then the bridge vaults are created.
  const collectionVaultPda = findVaultPda(umi, {
    mint: collectionMint.publicKey,
  });
  const collectionVault = await fetchVault(umi, collectionVaultPda);

  const assetVaultPda = findVaultPda(umi, { mint: itemMint.publicKey });
  const assetVault = await fetchVault(umi, assetVaultPda);

  t.like(collectionVault, <Vault>{
    discriminator: BridgeDiscriminator.Vault,
    state: BridgeState.Idle,
    mint: collectionMint.publicKey,
  });

  t.like(assetVault, <Vault>{
    discriminator: BridgeDiscriminator.Vault,
    state: BridgeState.Idle,
    mint: itemMint.publicKey,
  });

  // Derive both asset pubkeys
  const collectionAsset = umi.eddsa.findPda(BRIDGE_PROGRAM_ID, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(collectionMint.publicKey),
  ]);

  const itemAsset = umi.eddsa.findPda(BRIDGE_PROGRAM_ID, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(itemMint.publicKey),
  ]);

  // Update the item to be a member of the group.
  await group(umi, {
    asset: itemAsset,
    group: collectionAsset,
  }).sendAndConfirm(umi);

  // create a PubkeyMatch constraint that will block the transfer to the owner.
  const constraint = pubkeyMatch(Account.Recipient, [publicKey(notOwner)]);

  const asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: itemMint.publicKey })
  );

  t.like(asset, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: assetVaultPda[0],
    authority: umi.identity.publicKey,
    extensions: [
      {
        // Was a NFT not pNFT so should have no royalties extension
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://asset.bridge',
      },
    ],
  });

  // Update the collectionAsset to have the new constraint
  const data = getExtensionSerializerFromType(
    ExtensionType.Royalties
  ).serialize(royalties({ basisPoints, constraint }));

  await update(umi, {
    asset: collectionAsset,
    payer: umi.identity,
    extension: {
      extensionType: ExtensionType.Royalties,
      length: data.length,
      data,
    },
  }).sendAndConfirm(umi);

  const updatedCollectionAsset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: collectionMint.publicKey })
  );

  t.like(updatedCollectionAsset, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: collectionVaultPda[0],
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://collection.bridge',
      },
      royalties({
        basisPoints,
        constraint,
      }),
    ],
  });

  // Bridging the asset should fail because the group asset has royalties constraints.
  const promise = bridge(umi, {
    mint: itemMint.publicKey,
    owner,
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, {
    message: /Assertion Failure/,
  });
});
