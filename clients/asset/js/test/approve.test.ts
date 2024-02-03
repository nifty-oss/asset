import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  create,
  approve,
  fetchAsset,
  DelegateInput,
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

  // When we approve a delegate.
  const authority = generateSigner(umi).publicKey;
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    delegateInput: {
      __kind: 'Some',
      roles: [DelegateRole.Transfer],
    } as DelegateInput,
  }).sendAndConfirm(umi);

  // Then the delegate is set.
  // Then the delegate is set.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
      roles: [DelegateRole.Transfer],
    },
  });
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

  const roles = [DelegateRole.Transfer, DelegateRole.Burn];

  // When we approve a delegate.
  const authority = generateSigner(umi).publicKey;
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    delegateInput: { __kind: 'Some', roles } as DelegateInput,
  }).sendAndConfirm(umi);

  // Then the delegate is set.
  // Then the delegate is set.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
      roles: [DelegateRole.Transfer, DelegateRole.Burn],
    },
  });
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

  // And we approve a delegate with a single role.
  const authority = generateSigner(umi).publicKey;
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    delegateInput: {
      __kind: 'Some',
      roles: [DelegateRole.Transfer],
    } as DelegateInput,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
      roles: [DelegateRole.Transfer],
    },
  });

  // When we set a new role to an existing delegate.
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    delegateInput: {
      __kind: 'Some',
      roles: [DelegateRole.Burn],
    } as DelegateInput,
  }).sendAndConfirm(umi);

  // Then the delegate have both roles active.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
      roles: [DelegateRole.Transfer, DelegateRole.Burn],
    },
  });
});

test('it sets all roles when passing in the All variant', async (t) => {
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

  const allRoles = [
    DelegateRole.Transfer,
    DelegateRole.Lock,
    DelegateRole.Burn,
  ];

  // When we approve a delegate with the All role
  const authority = generateSigner(umi).publicKey;
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    delegateInput: { __kind: 'All' } as DelegateInput,
  }).sendAndConfirm(umi);

  // Then the delegate is set with all roles.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    delegate: {
      address: authority,
      roles: allRoles,
    },
  });
});
