import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  approve,
  create,
  delegateInput,
  fetchAsset,
  revoke,
} from '../src';
import { createUmi } from './_setup';

test('an owner can revoke a delegate', async (t) => {
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

  // And set a delegate on it.
  const authority = generateSigner(umi).publicKey;

  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: authority,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer],
    }),
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: authority,
    },
  });

  // When the owner revokes the delegate.
  await revoke(umi, {
    asset: asset.publicKey,
    signer: owner,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer],
    }),
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
  const owner = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // And set a delegate on it.
  const authority = generateSigner(umi);

  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: authority.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer],
    }),
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
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer],
    }),
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
  const owner = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // And set a delegate on it.
  const authority = generateSigner(umi);

  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: authority.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer],
    }),
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
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer],
    }),
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Invalid authority/ });

  // And the delefate is not removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: authority.publicKey,
      roles: [DelegateRole.Transfer],
    },
  });
});

test('revoking with the "All" variant revokes all active roles', async (t) => {
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

  // And set a delegate on it.
  const authority = generateSigner(umi);

  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: authority.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer, DelegateRole.Burn, DelegateRole.Lock],
    }),
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
    delegateInput: delegateInput('All'),
  }).sendAndConfirm(umi);

  // Then the delegate is removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: null,
  });
});
