import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  create,
  approve,
  fetchAsset,
  revoke,
  DelegateInput,
} from '../src';
import { createUmi } from './_setup';

test('a holder can revoke a delegate', async (t) => {
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

  // And set a delegate on it.
  const authority = generateSigner(umi).publicKey;
  const delegateInput = {
    __kind: 'Some',
    roles: [DelegateRole.Transfer],
  } as DelegateInput;

  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    delegateInput,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: authority,
    },
  });

  // When the holder revokes the delegate.
  await revoke(umi, {
    asset: asset.publicKey,
    signer: holder,
    delegateInput,
  }).sendAndConfirm(umi);

  // Then the delegate is removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: null,
  });
});

test('a delegate can revoke itself', async (t) => {
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

  // And set a delegate on it.
  const authority = generateSigner(umi);
  const delegateInput = {
    __kind: 'Some',
    roles: [DelegateRole.Transfer],
  } as DelegateInput;

  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    delegateInput,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: authority.publicKey,
    },
  });

  // When the delegate revokes itself.
  await revoke(umi, {
    asset: asset.publicKey,
    signer: authority,
    delegateInput,
  }).sendAndConfirm(umi);

  // Then the delegate is removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: null,
  });
});

test('a random signer cannot revoke the delegate', async (t) => {
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

  // And set a delegate on it.
  const authority = generateSigner(umi);
  const delegateInput = {
    __kind: 'Some',
    roles: [DelegateRole.Transfer],
  } as DelegateInput;

  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    delegateInput,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: authority.publicKey,
    },
  });

  const randomSigner = generateSigner(umi);
  // When we try to revoke the delegate with a random signer.
  const promise = revoke(umi, {
    asset: asset.publicKey,
    signer: randomSigner,
    delegateInput,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Invalid authority/ });

  // And the delefate is not removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: authority.publicKey,
    },
  });
});

test('revoking with the All variant revokes all active roles', async (t) => {
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

  // And set a delegate on it.
  const authority = generateSigner(umi);
  const delegateInput = {
    __kind: 'Some',
    roles: [DelegateRole.Transfer, DelegateRole.Burn, DelegateRole.Lock],
  } as DelegateInput;

  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    delegateInput,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: authority.publicKey,
    },
  });

  // When the delegate revokes itself.
  await revoke(umi, {
    asset: asset.publicKey,
    signer: authority,
    delegateInput: { __kind: 'All' } as DelegateInput,
  }).sendAndConfirm(umi);

  // Then the delegate is removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: null,
  });
});
