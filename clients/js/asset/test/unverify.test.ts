import { generateSigner, transactionBuilder } from '@metaplex-foundation/umi';
import { httpDownloader } from '@metaplex-foundation/umi-downloader-http';
import test from 'ava';
import {
  Asset,
  ExtensionType,
  TypedExtension,
  create,
  creators,
  fetchAsset,
  initialize,
  unverify,
  verify,
} from '../src';
import { createUmi } from './_setup';

test('it can unverify a creator', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([{ address: umi.identity.publicKey, share: 100 }]),
  }).sendAndConfirm(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  // And the creator is verified.
  await verify(umi, {
    asset: asset.publicKey,
    creator: umi.identity,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: umi.identity.publicKey,
            verified: true,
            share: 100,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
        ],
      },
    ],
  });

  // When we unverify the creator.

  await unverify(umi, {
    asset: asset.publicKey,
    creator: umi.identity,
  }).sendAndConfirm(umi);

  // Then the creator is unverified.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: umi.identity.publicKey,
            verified: false,
            share: 100,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
        ],
      },
    ],
  });
});

test('it can unverify multiple creators', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And two creators.
  const creator1 = generateSigner(umi);
  const creator2 = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([
      { address: creator1.publicKey, share: 50 },
      { address: creator2.publicKey, share: 50 },
    ]),
  }).sendAndConfirm(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  // And we verify both creators.
  await transactionBuilder()
    .add(
      verify(umi, {
        asset: asset.publicKey,
        creator: creator1,
      })
    )
    .add(
      verify(umi, {
        asset: asset.publicKey,
        creator: creator2,
      })
    )
    .sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: creator1.publicKey,
            verified: true,
            share: 50,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
          {
            address: creator2.publicKey,
            verified: true,
            share: 50,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
        ],
      },
    ],
  });

  // When we unverify both creators.
  await transactionBuilder()
    .add(
      unverify(umi, {
        asset: asset.publicKey,
        creator: creator1,
      })
    )
    .add(
      unverify(umi, {
        asset: asset.publicKey,
        creator: creator2,
      })
    )
    .sendAndConfirm(umi);

  // Then both creators are unverified.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: creator1.publicKey,
            verified: false,
            share: 50,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
          {
            address: creator2.publicKey,
            verified: false,
            share: 50,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
        ],
      },
    ],
  });
});

test('it cannot unverify a wrong creator', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([{ address: umi.identity.publicKey, share: 100 }]),
  }).sendAndConfirm(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  // And we verify the creator.
  await verify(umi, {
    asset: asset.publicKey,
    creator: umi.identity,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: umi.identity.publicKey,
            verified: true,
            share: 100,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
        ],
      },
    ],
  });

  // When we try to unverify a wrong creator.
  const promise = unverify(umi, {
    asset: asset.publicKey,
    creator: generateSigner(umi),
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /invalid program argument/,
  });

  // And the creator is still verified.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: umi.identity.publicKey,
            verified: true,
            share: 100,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
        ],
      },
    ],
  });
});

test('it cannot unverify a creator without creators extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    name: 'Asset with creators',
    payer: umi.identity,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [] as TypedExtension[],
  });

  // When we try to unverify a creator.
  const promise = unverify(umi, {
    asset: asset.publicKey,
    creator: generateSigner(umi),
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Extension not found/,
  });
});
