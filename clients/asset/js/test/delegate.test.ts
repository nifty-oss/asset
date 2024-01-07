import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  create,
  delegate,
  fetchAsset,
  isActive,
} from '../src';
import { createUmi } from './_setup';

test('it can set a delegate with a single role', async (t) => {
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

  // When we set a delegate.
  const authority = generateSigner(umi).publicKey;
  await delegate(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    args: [DelegateRole.Transfer],
  }).sendAndConfirm(umi);

  // Then the delegate is set.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
    },
  });

  t.true(isActive(account.delegate, DelegateRole.Transfer));
});

test('it can set a delegate with multiple role', async (t) => {
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

  const args = [DelegateRole.Transfer, DelegateRole.Burn];

  // When we set a delegate.
  const authority = generateSigner(umi).publicKey;
  await delegate(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    args,
  }).sendAndConfirm(umi);

  // Then the delegate is set.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
    },
  });

  t.true(
    args
      .map((role) => isActive(account.delegate, role))
      .reduce((previous, current) => previous && current)
  );
});

test('it can set a new role to an existing delegate', async (t) => {
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

  // And we set a delegate with a single role.
  const authority = generateSigner(umi).publicKey;
  await delegate(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    args: [DelegateRole.Transfer],
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
    },
  });

  t.true(isActive(account.delegate, DelegateRole.Transfer));
  t.false(isActive(account.delegate, DelegateRole.Burn));

  // When we set a new role to an existing delegate.
  await delegate(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    args: [DelegateRole.Burn],
  }).sendAndConfirm(umi);

  // Then the delegate have both roles active.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
    },
  });

  t.true(isActive(account.delegate, DelegateRole.Transfer));
  t.true(isActive(account.delegate, DelegateRole.Burn));
});
