import { mplToolbox } from '@metaplex-foundation/mpl-toolbox';
import {
  generateSigner,
  percentAmount,
  publicKey,
} from '@metaplex-foundation/umi';
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import {
  publicKey as publicKeySerializer,
  string,
} from '@metaplex-foundation/umi/serializers';
import {
  Asset,
  Discriminator as AssetDiscriminator,
  Standard as AssetStandard,
  State as AssetState,
  ExtensionType,
  fetchAsset,
  group,
  grouping,
  niftyAsset,
  pubkeyMatch,
  royalties,
  update,
} from '@nifty-oss/asset';
import test from 'ava';
import {
  BRIDGE_PROGRAM_ID,
  Discriminator,
  State,
  Vault,
  bridge,
  create,
  fetchVault,
  findBridgeAssetPda,
  findVaultPda,
  niftyBridge,
} from '../src';
import { createProgrammableNft, createVerifiedNft } from './_setup';

const defaultCreateArgs = {
  isCollection: false,
};

const collectionCreateArgs = {
  isCollection: true,
};

const createUmi = async () =>
  (await basecreateUmi())
    .use(mplToolbox())
    .use(niftyBridge())
    .use(niftyAsset());

test('pubkeymatch failing blocks a transfer on a group asset', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();
  const owner = generateSigner(umi);
  const notOwner = generateSigner(umi);

  const basisPoints = BigInt(550);

  // Create a Token Metadata non-fungible representing a collection.
  const collectionMint = await createProgrammableNft(umi, {
    name: 'Bridge Collection',
    symbol: 'BA',
    uri: 'https://collection.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    isCollection: true,
    tokenOwner: owner.publicKey,
  });

  // Create the corresponding collection asset on the bridge.
  await create(umi, {
    mint: collectionMint.publicKey,
    updateAuthority: umi.identity,
    ...collectionCreateArgs,
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

  // Create the corresponding asset on the bridge.
  await create(umi, {
    mint: itemMint.publicKey,
    collection: findBridgeAssetPda(umi, { mint: collectionMint.publicKey }),
    updateAuthority: umi.identity,
    ...defaultCreateArgs,
  }).sendAndConfirm(umi);

  // Then the bridge vaults are created.
  const collectionVaultPda = findVaultPda(umi, {
    mint: collectionMint.publicKey,
  });
  const collectionVault = await fetchVault(umi, collectionVaultPda);

  const assetVaultPda = findVaultPda(umi, { mint: itemMint.publicKey });
  const assetVault = await fetchVault(umi, assetVaultPda);

  t.like(collectionVault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
    mint: collectionMint.publicKey,
  });

  t.like(assetVault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
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
  const constraint = pubkeyMatch('Recipient', [publicKey(notOwner)]);

  const asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: itemMint.publicKey })
  );

  t.like(asset, <Asset>{
    discriminator: AssetDiscriminator.Asset,
    state: AssetState.Unlocked,
    standard: AssetStandard.NonFungible,
    owner: assetVaultPda[0],
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
  await update(umi, {
    asset: collectionAsset,
    payer: umi.identity,
    extension: royalties(basisPoints, constraint),
  }).sendAndConfirm(umi);

  const updatedCollectionAsset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: collectionMint.publicKey })
  );

  t.like(updatedCollectionAsset, <Asset>{
    discriminator: AssetDiscriminator.Asset,
    state: AssetState.Unlocked,
    standard: AssetStandard.NonFungible,
    owner: collectionVaultPda[0],
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://collection.bridge',
      },
      grouping(0, 1), // 1 item in the group
      royalties(basisPoints, constraint),
    ],
  });

  // Bridging the item asset should fail because the group asset has royalties constraints.
  const promise = bridge(umi, {
    mint: itemMint.publicKey,
    owner,
    groupAsset: collectionAsset,
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, {
    message: /Assertion Failure/,
  });

  // And we check that the asset was not transferred.
  const untransferredAsset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: itemMint.publicKey })
  );

  t.like(untransferredAsset, <Asset>{
    owner: assetVaultPda[0],
  });

  // Now update the royalty extension on the group asset to have the recipient be the pubkey match
  const newConstraint = pubkeyMatch('Recipient', [publicKey(owner)]);

  await update(umi, {
    asset: collectionAsset,
    payer: umi.identity,
    extension: royalties(basisPoints, newConstraint),
  }).sendAndConfirm(umi);

  const finalCollectionAsset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: collectionMint.publicKey })
  );

  // It's successfully updated
  t.like(finalCollectionAsset, <Asset>{
    discriminator: AssetDiscriminator.Asset,
    state: AssetState.Unlocked,
    standard: AssetStandard.NonFungible,
    owner: collectionVaultPda[0],
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://collection.bridge',
      },
      grouping(0, 1), // 1 item in the group
      royalties(basisPoints, newConstraint),
    ],
  });

  // Bridging the asset should succeed.
  await bridge(umi, {
    mint: itemMint.publicKey,
    owner,
    groupAsset: collectionAsset,
  }).sendAndConfirm(umi);

  // And we check that the asset was transferred.
  const transferredAsset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: itemMint.publicKey })
  );

  t.like(transferredAsset, <Asset>{
    owner: owner.publicKey,
  });
});
