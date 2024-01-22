import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { attributes, close, create, initialize } from '../src';
import { createUmi } from './_setup';

test('it can close an uninitialized asset buffer account', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);

  // And we initialize an asset with one extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes([{ traitType: 'head', value: 'hat' }]),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we close the asset buffer.
  await close(umi, {
    buffer: asset,
    destination: umi.identity.publicKey,
  }).sendAndConfirm(umi);

  // Then the asset buffer account was closed.
  t.false(await umi.rpc.accountExists(asset.publicKey), 'asset exists');
});

test('it cannot close an initialized asset account', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // When we try to close the asset account.
  const promise = close(umi, {
    buffer: asset,
    destination: umi.identity.publicKey,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /already initialized/ });
});
