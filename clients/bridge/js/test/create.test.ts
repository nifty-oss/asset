import { percentAmount, publicKey } from '@metaplex-foundation/umi';
import test from 'ava';
import {
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
  createUmi,
  createVerifiedNft,
} from './_setup';

test('it can create an asset on the bridge', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata non-fungible.
  const mint = await createNft(umi, {
    name: 'Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
  });

  // When we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // Then the birdge vault is created.
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

test('it cannot create an asset on the bridge without the update authority', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata non-fungible.
  const mint = await createNft(umi, {
    name: 'Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
  });

  // When we create the asset on the bridge.
  const promise = create(umi, {
    mint: publicKey(mint),
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, {
    message: /insufficient account keys for instruction/,
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
  }).sendAndConfirm(umi);

  // Then the birdge vault is created.
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
