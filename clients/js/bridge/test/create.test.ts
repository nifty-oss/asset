import {
  generateSigner,
  percentAmount,
  publicKey,
} from '@metaplex-foundation/umi';
import {
  publicKey as publicKeySerializer,
  string,
} from '@metaplex-foundation/umi/serializers';
import { Asset, ExtensionType, empty, fetchAsset } from '@nifty-oss/asset';
import test from 'ava';
import {
  BRIDGE_PROGRAM_ID,
  Discriminator,
  State,
  Vault,
  create,
  fetchVault,
  findBridgeAssetPda,
  findVaultPda,
} from '../src';
import {
  createCollectionNft,
  createNft,
  createProgrammableNft,
  createUmi,
  createVerifiedNft,
} from './_setup';

const defaultCreateArgs = {
  isCollection: false,
};

const collectionCreateArgs = {
  isCollection: true,
};

test('it can create an asset on the bridge', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata non-fungible.
  const mint = await createNft(umi, {
    name: 'Bridge Asset',
    symbol: 'BA',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
  });

  // When we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
    ...defaultCreateArgs,
  }).sendAndConfirm(umi);

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
    mint: mint.publicKey,
  });

  // And the asset is created.
  const asset = umi.eddsa.findPda(BRIDGE_PROGRAM_ID, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(mint.publicKey),
  ]);
  t.like(await fetchAsset(umi, asset), <Asset>{
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://asset.bridge',
      },
    ],
  });
});

test('it can create a collection asset on the bridge for a pNFT', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata programmable non-fungible.
  const mint = await createProgrammableNft(umi, {
    name: 'Bridge Asset',
    symbol: 'BA',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    mint: undefined,
  });

  // When we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
    ...collectionCreateArgs,
  }).sendAndConfirm(umi);

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
    mint: mint.publicKey,
  });

  // And the asset is created.
  const asset = umi.eddsa.findPda(BRIDGE_PROGRAM_ID, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(mint.publicKey),
  ]);

  // An empty constraint is added to the asset representing a 'pass-all' constraint.
  const constraint = empty();

  t.like(await fetchAsset(umi, asset), <Asset>{
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://asset.bridge',
      },
      {
        type: ExtensionType.Grouping,
        size: BigInt(0),
        maxSize: BigInt(0),
      },
      {
        type: ExtensionType.Royalties,
        basisPoints: BigInt(550),
        constraint,
      },
    ],
  });
});

test('it can create an asset on the bridge for a pNFT', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata programmable non-fungible.
  const mint = await createProgrammableNft(umi, {
    name: 'Bridge Asset',
    symbol: 'BA',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    mint: undefined,
  });

  // When we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
    ...defaultCreateArgs,
  }).sendAndConfirm(umi);

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
    mint: mint.publicKey,
  });

  // And the asset is created.
  const asset = umi.eddsa.findPda(BRIDGE_PROGRAM_ID, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(mint.publicKey),
  ]);

  t.like(await fetchAsset(umi, asset), <Asset>{
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://asset.bridge',
      },
      // Not a collection asset, so no royalties extension.
    ],
  });
});

test('it cannot create an asset on the bridge without the update authority as signer', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();
  const authority = generateSigner(umi);

  // And a Token Metadata non-fungible.
  const mint = await createNft(umi, {
    name: 'Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    authority,
    updateAuthority: authority,
    creators: [{ address: authority.publicKey, verified: true, share: 100 }],
  });

  // When we create the asset on the bridge.
  const promise = create(umi, {
    mint: publicKey(mint),
    updateAuthority: authority.publicKey,
    ...defaultCreateArgs,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, {
    message: /missing required signature/,
  });
});

test('it can create an asset on the bridge with a collection', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata non-fungible representing a collection.
  const collection = await createCollectionNft(umi, {
    name: 'Bridge Collection',
    uri: 'https://collection.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
  });

  // And we create the collection asset on the bridge.
  await create(umi, {
    mint: collection.publicKey,
    updateAuthority: umi.identity,
    ...defaultCreateArgs,
  }).sendAndConfirm(umi);

  // And we create a Token Metadata non-fungible representing an asset
  // from the collection.
  const mint = await createVerifiedNft(umi, {
    name: 'Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    collectionMint: collection.publicKey,
  });

  // When we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    collection: findBridgeAssetPda(umi, { mint: collection.publicKey }),
    updateAuthority: umi.identity,
    ...defaultCreateArgs,
  }).sendAndConfirm(umi);

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
    mint: mint.publicKey,
  });
});
