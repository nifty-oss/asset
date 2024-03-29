import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  State,
  approve,
  create,
  delegateInput,
  fetchAsset,
  lock,
  unlock,
} from '../src';
import { createUmi } from './_setup';

test('it can unlock an asset', async (t) => {
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

  // And we set a delegate that can lock the asset.
  const signer = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: signer.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Lock],
    }),
  }).sendAndConfirm(umi);

  // And we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    signer,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we unlock the asset.
  await unlock(umi, {
    asset: asset.publicKey,
    signer,
  }).sendAndConfirm(umi);

  // Then the asset is unlocked.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Unlocked,
  });
});

test('it cannot unlock an asset with an invalid delegate', async (t) => {
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

  // And we set a delegate that can lock the asset.
  const signer = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: signer.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Lock],
    }),
  }).sendAndConfirm(umi);

  // And we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    signer,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we unlock the asset with an invalid delegate.
  const fake = generateSigner(umi);
  const promise = unlock(umi, {
    asset: asset.publicKey,
    signer: fake,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Invalid delegate/ });

  // And the asset is still locked.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });
});

test('it cannot unlock an asset locked with a delegate as an owner', async (t) => {
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

  // And we set a delegate that can lock the asset.
  const signer = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: signer.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Lock],
    }),
  }).sendAndConfirm(umi);

  // And we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    signer,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we try unlock the asset as an owner.
  const promise = unlock(umi, {
    asset: asset.publicKey,
    signer: owner,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Invalid delegate/ });

  // And the asset is still locked.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });
});

test('it can unlock an asset as an owner', async (t) => {
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

  // And we lock the asset as an owner.
  await lock(umi, {
    asset: asset.publicKey,
    signer: owner,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we unlock the asset as an owner.
  await unlock(umi, {
    asset: asset.publicKey,
    signer: owner,
  }).sendAndConfirm(umi);

  // Then the asset is unlocked.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Unlocked,
  });
});

test('it can unlock an asset that is unlocked', async (t) => {
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

  // And we set a delegate that can lock the asset.
  const signer = generateSigner(umi);
  await approve(umi, {
    asset: asset.publicKey,
    owner,
    delegate: signer.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Lock],
    }),
  }).sendAndConfirm(umi);

  // When we unlock an unlocked asset.
  await unlock(umi, {
    asset: asset.publicKey,
    signer,
  }).sendAndConfirm(umi);

  // Then the asset is (still) unlocked.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Unlocked,
  });
});
