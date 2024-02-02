import { defaultPublicKey, generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  create,
  approve,
  fetchAsset,
  revoke,
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
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority,
    args: [DelegateRole.Transfer],
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
  }).sendAndConfirm(umi);

  // Then the delegate is removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: defaultPublicKey(),
    },
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
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    args: [DelegateRole.Transfer],
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
  }).sendAndConfirm(umi);

  // Then the delegate is removed.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    delegate: {
      address: defaultPublicKey(),
    },
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
  await approve(umi, {
    asset: asset.publicKey,
    holder,
    delegate: authority.publicKey,
    args: [DelegateRole.Transfer],
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
