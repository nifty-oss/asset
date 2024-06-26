/* eslint-disable no-plusplus */
import { generateSigner, transactionBuilder } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  create,
  fetchAsset,
  group,
  grouping,
  initialize,
  mint,
  update,
} from '../../src';
import { createUmi } from '../_setup';

test('it can create a new group asset with a maximum size', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize a metadata extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: grouping(10),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Group Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
      },
    ],
  });
});

test('it can create a new group asset of unlimited size', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize a metadata extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: grouping(),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Group Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 0n,
      },
    ],
  });
});

test('it can update the maximum size of an existing group', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    extensions: [grouping(10)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
      },
    ],
  });

  // And a "normal" asset.
  const asset = generateSigner(umi);
  await mint(umi, {
    asset,
    payer: umi.identity,
    name: 'Asset',
  }).sendAndConfirm(umi);

  // And we group the asset.
  await group(umi, {
    group: groupAsset.publicKey,
    asset: asset.publicKey,
  }).sendAndConfirm(umi);

  // And the group size has increased.
  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 1n,
        maxSize: 10n,
      },
    ],
  });

  // When we update the maximum size of the group.
  await update(umi, {
    asset: groupAsset.publicKey,
    payer: umi.identity,
    extension: grouping(1),
  }).sendAndConfirm(umi);

  // The the group maximum size has been updated.
  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 1n,
        maxSize: 1n,
      },
    ],
  });
});

test('it cannot reduce the size of a group', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    extensions: [grouping(10)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
      },
    ],
  });

  // And we mint and group assets.
  let builder = transactionBuilder();
  const SIZE = 5n;

  for (let i = 0; i < SIZE; i++) {
    const asset = generateSigner(umi);
    builder = builder.append(
      mint(umi, {
        asset,
        payer: umi.identity,
        group: groupAsset.publicKey,
        name: 'Asset',
      }).builders
    );
  }
  await builder.sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: SIZE,
        maxSize: 10n,
      },
    ],
  });

  // When we try to update the maximum size of the group to a smaller size.
  const promise = update(umi, {
    asset: groupAsset.publicKey,
    payer: umi.identity,
    extension: grouping(4),
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Extension data invalid/,
  });
});

test('it can update the size of a group to unlimited', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    extensions: [grouping(10)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
      },
    ],
  });

  // And we mint and group assets.
  let builder = transactionBuilder();
  const SIZE = 5n;

  for (let i = 0; i < SIZE; i++) {
    const asset = generateSigner(umi);
    builder = builder.append(
      mint(umi, {
        asset,
        payer: umi.identity,
        group: groupAsset.publicKey,
        name: 'Asset',
      }).builders
    );
  }
  await builder.sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: SIZE,
        maxSize: 10n,
      },
    ],
  });

  // When we update the maximum size of the group to unlimited.
  await update(umi, {
    asset: groupAsset.publicKey,
    payer: umi.identity,
    extension: grouping(),
  }).sendAndConfirm(umi);

  // The the group maximum size has been updated.
  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: SIZE,
        maxSize: 0n,
      },
    ],
  });
});

test('it can increase the size of a group', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    extensions: [grouping(10)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
      },
    ],
  });

  // And we mint and group assets.
  let builder = transactionBuilder();
  const SIZE = 5n;

  for (let i = 0; i < SIZE; i++) {
    const asset = generateSigner(umi);
    builder = builder.append(
      mint(umi, {
        asset,
        payer: umi.identity,
        group: groupAsset.publicKey,
        name: 'Asset',
      }).builders
    );
  }
  await builder.sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: SIZE,
        maxSize: 10n,
      },
    ],
  });

  // When we update the maximum size of the group.
  await update(umi, {
    asset: groupAsset.publicKey,
    payer: umi.identity,
    extension: grouping(1000),
  }).sendAndConfirm(umi);

  // The the group maximum size has been updated.
  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: SIZE,
        maxSize: 1000n,
      },
    ],
  });
});

test('it can add a delegate to the grouping on init', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  const delegate = generateSigner(umi);

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    extensions: [grouping(10, delegate.publicKey)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
        delegate: delegate.publicKey,
      },
    ],
  });

  // When we update the maximum size of the group.
  await update(umi, {
    asset: groupAsset.publicKey,
    payer: umi.identity,
    extension: grouping(100, delegate.publicKey),
  }).sendAndConfirm(umi);

  // Then the group maximum size has been updated, and the delegate is unchanged
  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 100n,
        delegate: delegate.publicKey,
      },
    ],
  });

  // When we remove the delegate.
  await update(umi, {
    asset: groupAsset.publicKey,
    payer: umi.identity,
    extension: grouping(100),
  }).sendAndConfirm(umi);

  // Then the delegate is removed
  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 100n,
        delegate: null,
      },
    ],
  });
});
