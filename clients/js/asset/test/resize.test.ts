import { assertAccountExists, generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { create, resize, sizeInput } from '../src';
import { createUmi } from './_setup';

test('it can resize an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // When we resize the asset.
  await resize(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    payer: umi.identity,
    sizeInput: sizeInput('Extend', { value: 1000 }),
  }).sendAndConfirm(umi);

  // Then an asset account was resized.
  const account = await umi.rpc.getAccount(asset.publicKey);
  assertAccountExists(account);

  t.true(account.data.length === 168 + 1000);
});

test('it can resize an asset to fit the minimum required size.', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // And we resize the asset to a size larger than the minimum required size.
  await resize(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    payer: umi.identity,
    sizeInput: sizeInput('Extend', { value: 1000 }),
  }).sendAndConfirm(umi);

  let account = await umi.rpc.getAccount(asset.publicKey);
  assertAccountExists(account);
  t.true(account.data.length === 168 + 1000);

  // When we resize the asset to fit the minimum required size.
  await resize(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    payer: umi.identity,
    sizeInput: sizeInput('Fit'),
  }).sendAndConfirm(umi);

  // Then an asset account was resized.
  account = await umi.rpc.getAccount(asset.publicKey);
  assertAccountExists(account);
  t.true(account.data.length === 168);
});
