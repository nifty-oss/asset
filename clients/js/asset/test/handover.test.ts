import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { Asset, create, fetchAsset, handover } from '../src';
import { createUmi } from './_setup';

test('it can handover an asset to a new authority', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const authority = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: umi.identity.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    authority,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    authority: authority.publicKey,
  });

  // When we handover the asset to a new authority.
  const newAuthority = generateSigner(umi);
  await handover(umi, {
    asset: asset.publicKey,
    authority,
    newAuthority,
  }).sendAndConfirm(umi);

  // Then the authority of the asset is updated.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    authority: newAuthority.publicKey,
  });
});

test('it cannot handover an asset with the wrong authority', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: umi.identity.publicKey,
    payer: umi.identity,
    authority: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    authority: umi.identity.publicKey,
  });

  // When we try handover the asset to a new authority with a fake authority.
  const fakeAuthority = generateSigner(umi);
  const newAuthority = generateSigner(umi);

  const promise = handover(umi, {
    asset: asset.publicKey,
    authority: fakeAuthority,
    newAuthority,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Invalid authority/ });
});
