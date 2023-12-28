import { generateSigner, publicKey, some } from '@metaplex-foundation/umi';
import { httpDownloader } from '@metaplex-foundation/umi-downloader-http';
import { findBufferPda, writeToBuffer } from '@metaplex-foundation/buffet';
import test from 'ava';
import {
  initialize,
  ExtensionType,
  getAttributesSerializer,
  findAssetPda,
} from '../src';
import { createUmi } from './_setup';

test('it can initialize a new account with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const mold = generateSigner(umi);

  // When we initialize the account with one extension.
  await initialize(umi, {
    mold,
    extensionType: ExtensionType.Attributes,
    data: some(
      getAttributesSerializer().serialize({
        traits: [{ traitType: 'head', value: 'hat' }],
      })
    ),
  }).sendAndConfirm(umi);

  // Then an account was created.
  t.true(
    await umi.rpc.accountExists(
      publicKey(findAssetPda(umi, { mold: mold.publicKey }))
    ),
    'account exists'
  );
});

test('it cannot initialize the same extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const mold = generateSigner(umi);

  // And we initialize the account with one extension.
  await initialize(umi, {
    mold,
    extensionType: ExtensionType.Attributes,
    data: some(
      getAttributesSerializer().serialize({
        traits: [{ traitType: 'head', value: 'hat' }],
      })
    ),
  }).sendAndConfirm(umi);

  // When we try to initialize the same extension again.
  const promise = initialize(umi, {
    mold,
    extensionType: ExtensionType.Attributes,
    data: some(
      getAttributesSerializer().serialize({
        traits: [{ traitType: 'power', value: 'wizard' }],
      })
    ),
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, { message: /Asset already initialized/ });
});

test('it can initialize a new account with multiple extensions', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  umi.use(httpDownloader());
  const mold = generateSigner(umi);

  // And we initialize the account with one extension.
  await initialize(umi, {
    mold,
    extensionType: ExtensionType.Attributes,
    data: some(
      getAttributesSerializer().serialize({
        traits: [{ traitType: 'head', value: 'hat' }],
      })
    ),
  }).sendAndConfirm(umi);

  const image = (
    await umi.downloader.download([
      'https://arweave.net/Dwf8_jphHptJkT9-hGVVUgqEZA2LSOSU9wD_Da-MfSQ',
    ])
  )[0].buffer;

  // And we copy the contents of the image into the buffer account.
  await writeToBuffer(umi, {
    authority: mold,
    data: new Uint8Array(image),
  }).sendAndConfirm(umi);

  const bufferPda = publicKey(
    findBufferPda(umi, { authority: mold.publicKey })
  );

  // When we try to initialize the same extension again.
  await initialize(umi, {
    mold,
    extensionType: ExtensionType.Image,
    buffer: publicKey(bufferPda),
  }).sendAndConfirm(umi);

  // Then an account was created.
  t.true(
    await umi.rpc.accountExists(
      publicKey(findAssetPda(umi, { mold: mold.publicKey }))
    ),
    'account exists'
  );
});
