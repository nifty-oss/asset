import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import { httpDownloader } from '@metaplex-foundation/umi-downloader-http';
import test from 'ava';
import {
  attributes,
  findAssetPda,
  image,
  initialize
} from '../src';
import { createUmi } from './_setup';

test('it can initialize a new account with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const canvas = generateSigner(umi);

  // When we initialize the account with one extension.
  await initialize(umi, {
    canvas,
    extension: attributes({
      traits: [{ traitType: 'head', value: 'hat' }],
    }),
  }).sendAndConfirm(umi);

  // Then an account was created.
  t.true(
    await umi.rpc.accountExists(
      publicKey(findAssetPda(umi, { canvas: canvas.publicKey }))
    ),
    'account exists'
  );
});

test('it cannot initialize the same extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const canvas = generateSigner(umi);

  // And we initialize the account with one extension.
  await initialize(umi, {
    canvas,
    extension: attributes({
      traits: [{ traitType: 'head', value: 'hat' }],
    }),
  }).sendAndConfirm(umi);

  // When we try to initialize the same extension again.
  const promise = initialize(umi, {
    canvas,
    extension: attributes({
      traits: [{ traitType: 'power', value: 'wizard' }],
    }),
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, { message: /Asset already initialized/ });
});

test('it can initialize a new account with multiple extensions', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  umi.use(httpDownloader());
  const canvas = generateSigner(umi);

  // And we initialize the account with one extension.
  await initialize(umi, {
    canvas,
    extension: attributes({
      traits: [{ traitType: 'head', value: 'hat' }],
    }),
  }).sendAndConfirm(umi);

  const imageData = (
    await umi.downloader.download([
      'https://arweave.net/Dwf8_jphHptJkT9-hGVVUgqEZA2LSOSU9wD_Da-MfSQ',
    ])
  )[0].buffer;

  // When we try to initialize the same extension again.
  await initialize(umi, {
    canvas,
    extension: image({ data: Array.from(imageData) }),
  }).sendAndConfirm(umi);

  // Then an account was created.
  t.true(
    await umi.rpc.accountExists(
      publicKey(findAssetPda(umi, { canvas: canvas.publicKey }))
    ),
    'account exists'
  );
});
