import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { DelegateRole, create, delegate, fetchAsset, transfer } from '../src';
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

test('it can transfer an asset as a delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const holderSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    holder: holderSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the holder is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // Now we delegate transfer authority of the asset
  await delegate(umi, {
    asset: assetSigner.publicKey,
    holder: holderSigner,
    delegate: delegateSigner.publicKey,
    args: [DelegateRole.Transfer],
  }).sendAndConfirm(umi);

  // and transfer the asset as the delegate.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: delegateSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred.
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === recipient);
});

test('invalid signer cannot transfer', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const holderSigner = generateSigner(umi);
  const invalidSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset
  await create(umi, {
    asset: assetSigner,
    holder: holderSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the holder is correct.
  const asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // We transfer the asset with an invalid signer
  const result = transfer(umi, {
    asset: assetSigner.publicKey,
    signer: invalidSigner,
    recipient,
  }).sendAndConfirm(umi);

  // and it fails.
  await t.throwsAsync(result, {
    message: /Invalid holder or transfer delegate/,
  });
});

test('holder transfer clears delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const holderSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    holder: holderSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the holder is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // Now we set a delegate on the asset
  await delegate(umi, {
    asset: assetSigner.publicKey,
    holder: holderSigner,
    delegate: delegateSigner.publicKey,
    args: [DelegateRole.Lock, DelegateRole.Transfer],
  }).sendAndConfirm(umi);

  // and transfer the asset as holder.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: holderSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === recipient);

  // and the delegate is cleared.
  asset.delegate === null;
});

test('delegate transfer clears delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const holderSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    holder: holderSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the holder is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // Now we set a delegate on the asset
  await delegate(umi, {
    asset: assetSigner.publicKey,
    holder: holderSigner,
    delegate: delegateSigner.publicKey,
    args: [DelegateRole.Lock, DelegateRole.Transfer],
  }).sendAndConfirm(umi);

  // and transfer the asset as the delegate.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: delegateSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === recipient);

  // and the delegate is cleared.
  asset.delegate === null;
});

test('self-transfer does not clear delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const holderSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    holder: holderSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the holder is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // Now we set a delegate on the asset
  await delegate(umi, {
    asset: assetSigner.publicKey,
    holder: holderSigner,
    delegate: delegateSigner.publicKey,
    args: [DelegateRole.Burn],
  }).sendAndConfirm(umi);

  // and transfer the asset as holder to itself.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: holderSigner,
    recipient: holderSigner.publicKey,
  }).sendAndConfirm(umi);

  // It was transferred
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.holder === holderSigner.publicKey);

  // and the delegate still exists.
  asset.delegate.address === delegateSigner.publicKey;
});
