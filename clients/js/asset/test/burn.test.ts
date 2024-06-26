import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  approve,
  burn,
  create,
  delegateInput,
  fetchAsset,
  grouping,
  mint,
} from '../src';
import { createUmi } from './_setup';

test('it can burn an asset as an owner', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const assetSigner = generateSigner(umi);
  const ownerSigner = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset: assetSigner,
    owner: ownerSigner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, assetSigner.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: ownerSigner.publicKey,
    authority: umi.identity.publicKey,
  });

  // Burn the asset.
  await burn(umi, {
    asset: assetSigner.publicKey,
    signer: ownerSigner,
  }).sendAndConfirm(umi);

  // Then the asset is gone.
  t.false(await umi.rpc.accountExists(assetSigner.publicKey), 'asset exists');
});

test('it can burn an asset as a delegate', async (t) => {
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

  // Now we delegate burn authority of the asset
  await approve(umi, {
    asset: assetSigner.publicKey,
    owner: ownerSigner,
    delegate: delegateSigner.publicKey,
    delegateInput: delegateInput('Some', {
      roles: [DelegateRole.Burn],
    }),
  }).sendAndConfirm(umi);

  // and burn the asset as the delegate.
  await burn(umi, {
    asset: assetSigner.publicKey,
    signer: delegateSigner,
  }).sendAndConfirm(umi);

  // Then the asset is gone.
  t.false(await umi.rpc.accountExists(assetSigner.publicKey), 'asset exists');
});

test('invalid signer cannot burn', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);
  const invalid = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
  });

  // Burn the asset.
  const promise = burn(umi, {
    asset: asset.publicKey,
    signer: invalid,
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, {
    message: /Invalid owner or burn delegate/,
  });

  // Then the asset still exists.
  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');
});

test('it ungroups an asset on burn', async (t) => {
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

  // When we burn the asset.
  await burn(umi, {
    asset: asset.publicKey,
    signer: umi.identity,
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  // Then the asset is gone.
  t.false(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

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

test('it cannot burn a grouped asset without group account', async (t) => {
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

  // When we try to burn the asset without the group account.
  const promise = burn(umi, {
    asset: asset.publicKey,
    signer: umi.identity,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /insufficient account keys for instruction/,
  });
});

test('it cannot burn a non-empty group asset', async (t) => {
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

  // When we try to burn the group asset.
  const promise = burn(umi, {
    asset: groupAsset.publicKey,
    signer: umi.identity,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Group is not empty/,
  });
});

test('it can burn an empty group asset', async (t) => {
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

  // When we burn the empty group asset.
  await burn(umi, {
    asset: groupAsset.publicKey,
    signer: umi.identity,
  }).sendAndConfirm(umi);

  // Then the group asset is gone.
  t.false(await umi.rpc.accountExists(groupAsset.publicKey), 'asset exists');
});
