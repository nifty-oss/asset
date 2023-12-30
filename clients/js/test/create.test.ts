import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  attributes,
  create,
  fetchAsset,
  findAssetPda,
  initialize,
} from '../src';
import { createUmi } from './_setup';

test('it can create a account', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const holder = generateSigner(umi);
  const canvas = generateSigner(umi);

  // When we create a new account.
  await create(umi, {
    holder: holder.publicKey,
    canvas,
    name: 'Digital Asset',
    symbol: 'DA',
  }).sendAndConfirm(umi);

  // Then an account was created with the correct data.
  t.like(
    await fetchAsset(umi, findAssetPda(umi, { canvas: canvas.publicKey })),
    <Asset>{
      holder: holder.publicKey,
      authority: umi.identity.publicKey,
    }
  );
});

test('it can create a new account with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const holder = generateSigner(umi);
  const canvas = generateSigner(umi);

  // And we initialize an extension.
  await initialize(umi, {
    canvas,
    extension: attributes({ traits: [{ traitType: 'head', value: 'hat' }] }),
  }).sendAndConfirm(umi);

  // When we create a new account.
  await create(umi, {
    holder: holder.publicKey,
    canvas,
    name: 'Digital Asset',
    symbol: 'DA',
  }).sendAndConfirm(umi);

  // Then an account was created with the correct data.
  t.like(
    await fetchAsset(umi, findAssetPda(umi, { canvas: canvas.publicKey })),
    <Asset>{
      holder: holder.publicKey,
      authority: umi.identity.publicKey,
    }
  );
});
