import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  State,
  create,
  approve,
  fetchAsset,
  lock,
  transfer,
} from '../src';
import { createUmi } from './_setup';

test('it can lock an asset as a holder', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // When we lock the asset as a holder.
  await lock(umi, {
    asset: asset.publicKey,
    authority: holder,
  }).sendAndConfirm(umi);

  // Then the asset is locked.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });
});

test('it can lock an asset with a delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // And we set a delegate that can lock the asset.
  const authority = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    args: [DelegateRole.Lock],
  }).sendAndConfirm(umi);

  // When we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    authority,
  }).sendAndConfirm(umi);

  // Then the asset is locked.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });
});

test('it cannot transfer a locked asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // And we set a delegate that can lock the asset.
  const authority = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    args: [DelegateRole.Lock],
  }).sendAndConfirm(umi);

  // And we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    authority,
  }).sendAndConfirm(umi);

  // When we try to transfer the asset
  const recipient = generateSigner(umi).publicKey;
  const promise = transfer(umi, {
    asset: asset.publicKey,
    signer: holder,
    recipient,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Asset is locked/ });
});

test('it cannot lock an asset without "Lock" role', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // And we set a delegate that can lock the asset.
  const authority = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    args: [DelegateRole.Transfer],
  }).sendAndConfirm(umi);

  // When we lock the asset.
  const promise = lock(umi, {
    asset: asset.publicKey,
    authority,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Delegate role not active/ });
});

test('it cannot lock as a holder if delegate set', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // And we set a delegate that can lock the asset.
  const authority = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    args: [DelegateRole.Transfer, DelegateRole.Lock],
  }).sendAndConfirm(umi);

  // When we try to lock the asset as a holder.
  const promise = lock(umi, {
    asset: asset.publicKey,
    authority: holder,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Invalid delegate/ });
});
