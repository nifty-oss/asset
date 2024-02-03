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
  unlock,
  DelegateInput,
} from '../src';
import { createUmi } from './_setup';

test('it can unlock an asset', async (t) => {
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
    delegateInput: {
      __kind: 'Some',
      roles: [DelegateRole.Lock],
    } as DelegateInput,
  }).sendAndConfirm(umi);

  // And we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    authority,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we unlock the asset.
  await unlock(umi, {
    asset: asset.publicKey,
    authority,
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
    delegateInput: {
      __kind: 'Some',
      roles: [DelegateRole.Lock],
    } as DelegateInput,
  }).sendAndConfirm(umi);

  // And we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    authority,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we unlock the asset with an invalid delegate.
  const fake = generateSigner(umi);
  const promise = unlock(umi, {
    asset: asset.publicKey,
    authority: fake,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Invalid delegate/ });

  // And the asset is still locked.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });
});

test('it cannot unlock an asset locked with a delegate as a holder', async (t) => {
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
    delegateInput: {
      __kind: 'Some',
      roles: [DelegateRole.Lock],
    } as DelegateInput,
  }).sendAndConfirm(umi);

  // And we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    authority,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we try unlock the asset as a holder.
  const promise = unlock(umi, {
    asset: asset.publicKey,
    authority: holder,
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, { message: /Invalid delegate/ });

  // And the asset is still locked.
  account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });
});

test('it can unlock an asset as a holder', async (t) => {
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

  // And we lock the asset as a holder.
  await lock(umi, {
    asset: asset.publicKey,
    authority: holder,
  }).sendAndConfirm(umi);

  let account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Locked,
  });

  // When we unlock the asset as a holder.
  await unlock(umi, {
    asset: asset.publicKey,
    authority: holder,
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
    delegateInput: {
      __kind: 'Some',
      roles: [DelegateRole.Lock],
    } as DelegateInput,
  }).sendAndConfirm(umi);

  // When we unlock an unlocked asset.
  await unlock(umi, {
    asset: asset.publicKey,
    authority,
  }).sendAndConfirm(umi);

  // Then the asset is (still) unlocked.
  const account = await fetchAsset(umi, asset.publicKey);
  t.like(account, <Asset>{
    state: State.Unlocked,
  });
});
