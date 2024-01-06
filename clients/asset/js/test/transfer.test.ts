import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { create, fetchAsset, transfer } from '../src';
import { createUmi } from './_setup';

test('it can transfer an asset as a holder', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const holderSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    holder: holderSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Holder is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // And we transfer the asset.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: holderSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred.
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === recipient);
});

// TODO: Need delegate handler implemented.
// test('it can transfer an asset as a transfer delegate', async (t) => {
//   // Given a Umi instance and a new signer.
//   const umi = await createUmi();
//   const assetSigner = generateSigner(umi);
//   const holderSigner = generateSigner(umi);
//   const delegateSigner = generateSigner(umi);
//   const recipient = generateSigner(umi).publicKey;

//   // When we create a new asset.
//   await create(umi, {
//     asset: assetSigner,
//     holder: holderSigner.publicKey,
//     payer: umi.identity,
//     name: 'Digital Asset',
//   }).sendAndConfirm(umi);

//   // Holder is correct.
//   let asset = await fetchAsset(umi, assetSigner.publicKey);
//   t.true(asset.holder === holderSigner.publicKey);

//   // And we transfer the asset.
//   await transfer(umi, {
//     asset: assetSigner.publicKey,
//     signer: holderSigner,
//     recipient,
//   }).sendAndConfirm(umi);

//   // It was transferred.
//   asset = await fetchAsset(umi, assetSigner.publicKey);
//   t.true(asset.holder === recipient);
// });

test('invalid signer cannot transfer', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const holderSigner = generateSigner(umi);
  const invalidSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    holder: holderSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Holder is correct.
  const asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // And we transfer the asset with an invalid signer.
  const result = transfer(umi, {
    asset: assetSigner.publicKey,
    signer: invalidSigner,
    recipient,
  }).sendAndConfirm(umi);

  await t.throwsAsync(result, {
    message: /Invalid holder or transfer delegate/,
  });
});
