import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  create,
  fetchAsset,
  initialize,
  lock,
  manager,
  unlock,
} from '../../src';
import { createUmi } from '../_setup';

test('it can create a new managed asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an asset with a managed extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: manager(umi.identity.publicKey, DelegateRole.Transfer),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Managed Asset',
    standard: Standard.Managed,
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Managed,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Manager,
        delegate: {
          address: umi.identity.publicKey,
          roles: [DelegateRole.Transfer],
        },
      },
    ],
  });
});

test('it can create a new managed asset with multiple delegate roles', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an asset with a managed extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: manager(umi.identity.publicKey, [
      DelegateRole.Transfer,
      DelegateRole.Burn,
    ]),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Managed Asset',
    standard: Standard.Managed,
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Managed,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Manager,
        delegate: {
          address: umi.identity.publicKey,
          roles: [DelegateRole.Transfer, DelegateRole.Burn],
        },
      },
    ],
  });
});

test('it cannot create a new managed asset without the manager extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // When we create try to create a managed asset without the extension.
  const promise = create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Managed Asset',
    standard: Standard.Managed,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Extension data invalid/ });
});

test('it cannot create a non-managed asset with the manager extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an asset with a managed extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: manager(umi.identity.publicKey, DelegateRole.Transfer),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create try to create a managed asset without the extension.
  const promise = create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Non-Managed Asset',
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Extension data invalid/ });
});

test('it can lock an asset with the manager delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);
  const delegate = generateSigner(umi);

  // And we initialize an asset with a managed extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: manager(delegate.publicKey, [
      DelegateRole.Transfer,
      DelegateRole.Lock,
      DelegateRole.Burn,
    ]),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // And we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Managed Asset',
    standard: Standard.Managed,
  }).sendAndConfirm(umi);

  // When we lock the asset.
  await lock(umi, {
    asset: asset.publicKey,
    signer: delegate,
  }).sendAndConfirm(umi);

  // Then an asset is locked.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Locked,
    standard: Standard.Managed,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    delegate: null,
    extensions: [
      {
        type: ExtensionType.Manager,
        delegate: {
          address: delegate.publicKey,
          roles: [DelegateRole.Transfer, DelegateRole.Lock, DelegateRole.Burn],
        },
      },
    ],
  });
});

test('it can unlock an asset with the manager delegate', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);
  const delegate = generateSigner(umi);

  // And we initialize an asset with a managed extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: manager(delegate.publicKey, [
      DelegateRole.Transfer,
      DelegateRole.Lock,
      DelegateRole.Burn,
    ]),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // And we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Managed Asset',
    standard: Standard.Managed,
  }).sendAndConfirm(umi);

  // And we lock the asset as the owner.
  await lock(umi, {
    asset: asset.publicKey,
    signer: owner,
  }).sendAndConfirm(umi);

  let assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Locked,
    standard: Standard.Managed,
    owner: owner.publicKey,
  });

  // When we unlock the asset with the manager delegate.
  await unlock(umi, {
    asset: asset.publicKey,
    signer: delegate,
  }).sendAndConfirm(umi);

  // Then an asset is unlocked.
  assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Managed,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    delegate: null,
    extensions: [
      {
        type: ExtensionType.Manager,
        delegate: {
          address: delegate.publicKey,
          roles: [DelegateRole.Transfer, DelegateRole.Lock, DelegateRole.Burn],
        },
      },
    ],
  });
});
