import { generateSigner } from '@metaplex-foundation/umi';
import { httpDownloader } from '@metaplex-foundation/umi-downloader-http';
import test from 'ava';
import { attributes, image, initialize, write } from '../src';
import { createUmi } from './_setup';

test('it can initialize a new asset with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);

  // When we initialize an asset with one extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes({
      traits: [{ traitType: 'head', value: 'hat' }],
    }),
  }).sendAndConfirm(umi);

  // Then the asset account was created.
  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');
});

test('it cannot initialize the same extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);

  // And we initialize an asset with one extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes({
      traits: [{ traitType: 'head', value: 'hat' }],
    }),
  }).sendAndConfirm(umi);

  // When we try to initialize the same extension again.
  const promise = initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes({
      traits: [{ traitType: 'power', value: 'wizard' }],
    }),
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, { message: /Asset already initialized/ });
});

test('it can initialize a new asset with multiple extensions', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);

  // And we initialize an asset with an attributes  extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes({
      traits: [{ traitType: 'head', value: 'hat' }],
    }),
  }).sendAndConfirm(umi);

  const imageData = (
    await umi.downloader.download([
      'https://arweave.net/Dwf8_jphHptJkT9-hGVVUgqEZA2LSOSU9wD_Da-MfSQ',
    ])
  )[0].buffer;

  // And we initialize an image extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: image({ length: imageData.length }),
  }).sendAndConfirm(umi);

  // When we write the extension data.
  await write(umi, {
    asset,
    data: new Uint8Array(imageData),
  }).sendAndConfirm(umi);

  // Then an account was created.
  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');
});
