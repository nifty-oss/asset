import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  ExtensionType,
  fetchAsset,
  group,
  grouping,
  mint,
} from '../src';
import { createUmi } from './_setup';

test('it can group an asset', async (t) => {
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

  // When we group the asset.
  await group(umi, {
    group: groupAsset.publicKey,
    asset: asset.publicKey,
  }).sendAndConfirm(umi);

  // Then the group is set on the asset.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    group: groupAsset.publicKey,
  });

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
});

test('it cannot exceed max group size', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    extensions: [grouping(2)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 2n,
      },
    ],
  });

  // And a "normal" asset.
  const asset1 = generateSigner(umi);
  await mint(umi, {
    asset: asset1,
    payer: umi.identity,
    name: 'Asset',
  }).sendAndConfirm(umi);

  // Added to the group.
  await group(umi, {
    group: groupAsset.publicKey,
    asset: asset1.publicKey,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset1.publicKey), <Asset>{
    group: groupAsset.publicKey,
  });

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 1n,
        maxSize: 2n,
      },
    ],
  });

  // And we create a second "normal" asset.
  const asset2 = generateSigner(umi);
  await mint(umi, {
    asset: asset2,
    payer: umi.identity,
    name: 'Asset',
  }).sendAndConfirm(umi);

  // Added to the same group.
  await group(umi, {
    group: groupAsset.publicKey,
    asset: asset2.publicKey,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset2.publicKey), <Asset>{
    group: groupAsset.publicKey,
  });

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 2n,
        maxSize: 2n,
      },
    ],
  });

  // And we create a third "normal" asset.
  const asset3 = generateSigner(umi);
  await mint(umi, {
    asset: asset3,
    payer: umi.identity,
    name: 'Asset',
  }).sendAndConfirm(umi);

  // When we try to group the asset.
  const promise = group(umi, {
    group: groupAsset.publicKey,
    asset: asset3.publicKey,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Maximum group size reached/ });

  // And the group size is the same.
  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 2n,
        maxSize: 2n,
      },
    ],
  });
});

test('it cannot replace the group of an asset', async (t) => {
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

  // And we add the asset to the group.
  await group(umi, {
    group: groupAsset.publicKey,
    asset: asset.publicKey,
  }).sendAndConfirm(umi);

  // Then we create a second group asset.

  // Then the group is set on the asset.
  const secondGroup = generateSigner(umi);
  await mint(umi, {
    asset: secondGroup,
    payer: umi.identity,
    name: 'Group',
    extensions: [grouping(10)],
  }).sendAndConfirm(umi);

  // When we try to add the assset to the second group.
  const promise = group(umi, {
    group: secondGroup.publicKey,
    asset: asset.publicKey,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Asset is already in a group/ });

  // And the group of the asset does not change.

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    group: groupAsset.publicKey,
  });
});

test('it can be grouped using a group delegate', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  const delegate = generateSigner(umi);
  const authority = generateSigner(umi).publicKey;

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    authority,
    extensions: [grouping(10, delegate.publicKey)],
  }).sendAndConfirm(umi);

  // And a "normal" asset.
  const asset = generateSigner(umi);
  await mint(umi, {
    asset,
    payer: umi.identity,
    authority,
    name: 'Asset',
  }).sendAndConfirm(umi);

  // Then we can group the assets using a delegate signer
  await group(umi, {
    asset: asset.publicKey,
    authority: delegate,
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    group: groupAsset.publicKey,
  });
});
