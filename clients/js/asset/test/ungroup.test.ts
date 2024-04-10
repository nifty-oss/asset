import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  ExtensionType,
  fetchAsset,
  grouping,
  mint,
  ungroup,
} from '../src';
import { createUmi } from './_setup';

test('it can ungroup an asset', async (t) => {
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
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    group: groupAsset.publicKey,
  });

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

  // When we ungroup the asset.
  await ungroup(umi, {
    asset: asset.publicKey,
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  // Then the group is removed from the asset.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    group: null,
  });

  // And the group size has decreased.
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
});

test('It can ungroup an asset as a group delegate', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  const delegate = generateSigner(umi);
  const authority = generateSigner(umi);

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await mint(umi, {
    asset: groupAsset,
    payer: umi.identity,
    name: 'Group',
    authority,
    extensions: [grouping(10, delegate.publicKey)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    authority: authority.publicKey,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
        delegate: delegate.publicKey,
      },
    ],
  });

  // And a "normal" asset.
  const asset = generateSigner(umi);
  await mint(umi, {
    asset,
    payer: umi.identity,
    name: 'Asset',
    authority,
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    group: groupAsset.publicKey,
  });

  t.like(await fetchAsset(umi, groupAsset.publicKey), <Asset>{
    group: null,
    authority: authority.publicKey,
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 1n,
        maxSize: 10n,
        delegate: delegate.publicKey,
      },
    ],
  });

  // When we ungroup the asset as the delegate.
  await ungroup(umi, {
    asset: asset.publicKey,
    group: groupAsset.publicKey,
    authority: delegate,
  }).sendAndConfirm(umi);

  // Then the group is removed from the asset.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    group: null,
  });

  // And the group size has decreased.
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
});
