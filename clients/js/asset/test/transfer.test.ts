import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  Standard,
  approve,
  create,
  delegateInput,
  fetchAsset,
  mint,
  manager,
  transfer,
} from '../src';
import { createUmi } from './_setup';

test('it can transfer an asset as an owner', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Owner is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // And we transfer the asset.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: ownerSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred.
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === recipient);
});

test('it can transfer an asset as a delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the owner is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // Now we delegate transfer authority of the asset
  await approve(umi, {
    asset: assetSigner.publicKey,
    owner: ownerSigner,
    delegate: delegateSigner.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Transfer],
    }),
  }).sendAndConfirm(umi);

  // and transfer the asset as the delegate.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: delegateSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred.
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === recipient);
});

test('invalid signer cannot transfer', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const invalidSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the owner is correct.
  const asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // We transfer the asset with an invalid signer
  const result = transfer(umi, {
    asset: assetSigner.publicKey,
    signer: invalidSigner,
    recipient,
  }).sendAndConfirm(umi);

  // and it fails.
  await t.throwsAsync(result, {
    message: /Invalid owner or transfer delegate/,
  });
});

test('owner transfer clears delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the owner is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // Now we set a delegate on the asset
  await approve(umi, {
    asset: assetSigner.publicKey,
    owner: ownerSigner,
    delegate: delegateSigner.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Burn, DelegateRole.Transfer],
    }),
  }).sendAndConfirm(umi);

  // and transfer the asset as owner.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: ownerSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === recipient);

  // and the delegate is cleared.
  t.true(asset.delegate === null);
});

test('delegate transfer clears delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the owner is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // Now we set a delegate on the asset
  await approve(umi, {
    asset: assetSigner.publicKey,
    owner: ownerSigner,
    delegate: delegateSigner.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Lock, DelegateRole.Transfer],
    }),
  }).sendAndConfirm(umi);

  // and transfer the asset as the delegate.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: delegateSigner,
    recipient,
  }).sendAndConfirm(umi);

  // It was transferred
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === recipient);

  // and the delegate is cleared.
  t.true(asset.delegate === null);
});

test('self-transfer does not clear delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // the owner is correct.
  const asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // Now we set a delegate on the asset
  await approve(umi, {
    asset: assetSigner.publicKey,
    owner: ownerSigner,
    delegate: delegateSigner.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Burn],
    }),
  }).sendAndConfirm(umi);

  // and transfer the asset as owner to itself.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: ownerSigner,
    recipient: ownerSigner.publicKey,
  }).sendAndConfirm(umi);

  // It was transferred and the delegate is still set.
  t.like(await fetchAsset(umi, assetSigner.publicKey), <Asset>{
    owner: ownerSigner.publicKey,
    delegate: {
      address: delegateSigner.publicKey,
    },
  });
});

test('it cannot transfer a soulbound asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Soulbound Asset',
    standard: Standard.Soulbound,
  }).sendAndConfirm(umi);

  // Owner is correct.
  const asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // When we try to transfer the asset.
  const recipient = generateSigner(umi).publicKey;
  const promise = transfer(umi, {
    asset: assetSigner.publicKey,
    signer: ownerSigner,
    recipient,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Soulbound assets are non-transferable/,
  });
});

test('it can transfer an asset as a manager delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new subscription asset.
  await mint(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Managed Asset',
    standard: Standard.Managed,
    extensions: [manager(delegateSigner.publicKey, DelegateRole.Transfer)],
  }).sendAndConfirm(umi);

  // the owner is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // When we transfer the asset as the delegate.
  await transfer(umi, {
    asset: assetSigner.publicKey,
    signer: delegateSigner,
    recipient,
  }).sendAndConfirm(umi);

  // Then the asset was transferred.
  asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === recipient);
});

test('it cannot transfer an asset with the wrong manager delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);
  const delegateSigner = generateSigner(umi);
  const recipient = generateSigner(umi).publicKey;

  // When we create a new subscription asset.
  await mint(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Managed Asset',
    standard: Standard.Managed,
    extensions: [manager(delegateSigner.publicKey, DelegateRole.Transfer)],
  }).sendAndConfirm(umi);

  // the owner is correct.
  let asset = await fetchAsset(umi, assetSigner.publicKey);
  t.true(asset.owner === ownerSigner.publicKey);

  // When we try to transfer the asset with the wrong delegate.
  const promise = transfer(umi, {
    asset: assetSigner.publicKey,
    signer: umi.identity,
    recipient,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Invalid owner or transfer delegate/,
  });
});
