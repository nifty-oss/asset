import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  DelegateRole,
  ExtensionType,
  Standard,
  attributes,
  create,
  empty,
  fetchAsset,
  grouping,
  links,
  manager,
  metadata,
  remove,
  royalties,
} from '../src';
import { createUmi } from './_setup';

test('it can remove an extension from an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new asset with an extension
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
    payer: umi.identity,
    extensions: [
      attributes([
        { name: 'Type', value: 'Dark' },
        { name: 'Clothes', value: 'Purple Shirt' },
        { name: 'Ears', value: 'None' },
      ]),
      metadata({
        symbol: 'MAD',
        uri: 'https://madlads.s3.us-west-2.amazonaws.com/json/8420.json',
      }),
      royalties(500),
    ],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        values: [
          { name: 'Type', value: 'Dark' },
          { name: 'Clothes', value: 'Purple Shirt' },
          { name: 'Ears', value: 'None' },
        ],
      },
      {
        type: ExtensionType.Royalties,
        basisPoints: 500n,
        constraint: empty(),
      },
      {
        type: ExtensionType.Metadata,
        symbol: 'MAD',
        uri: 'https://madlads.s3.us-west-2.amazonaws.com/json/8420.json',
      },
    ],
  });

  // When we remove the extension from the asset.
  await remove(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    recipient: umi.identity.publicKey,
    extensionType: ExtensionType.Royalties,
  }).sendAndConfirm(umi);

  // Then the extension is removed from the asset.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        values: [
          { name: 'Type', value: 'Dark' },
          { name: 'Clothes', value: 'Purple Shirt' },
          { name: 'Ears', value: 'None' },
        ],
      },
      {
        type: ExtensionType.Metadata,
        symbol: 'MAD',
        uri: 'https://madlads.s3.us-west-2.amazonaws.com/json/8420.json',
      },
    ],
  });
});

test('it can remove the first extension of an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new asset without extensions.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
    payer: umi.identity,
    extensions: [
      links([
        {
          name: 'metadata',
          uri: 'https://arweave.net/ebBV1qEYt65AKmM2J5wH_Vg-gjBa9YcwSYWFVt0rw9w',
        },
      ]),
      attributes([
        { name: 'Type', value: 'Dark' },
        { name: 'Clothes', value: 'Purple Shirt' },
        { name: 'Ears', value: 'None' },
      ]),
    ],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Links,
        values: [
          {
            name: 'metadata',
            uri: 'https://arweave.net/ebBV1qEYt65AKmM2J5wH_Vg-gjBa9YcwSYWFVt0rw9w',
          },
        ],
      },
      {
        type: ExtensionType.Attributes,
        values: [
          { name: 'Type', value: 'Dark' },
          { name: 'Clothes', value: 'Purple Shirt' },
          { name: 'Ears', value: 'None' },
        ],
      },
    ],
  });

  // When we remove the first extension from the asset.
  await remove(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    recipient: umi.identity.publicKey,
    extensionType: ExtensionType.Links,
  }).sendAndConfirm(umi);

  // Then the first extension is removed from the asset.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        values: [
          { name: 'Type', value: 'Dark' },
          { name: 'Clothes', value: 'Purple Shirt' },
          { name: 'Ears', value: 'None' },
        ],
      },
    ],
  });
});

test('it can remove the last extension of an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new asset without extensions.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
    payer: umi.identity,
    extensions: [
      links([
        {
          name: 'metadata',
          uri: 'https://arweave.net/ebBV1qEYt65AKmM2J5wH_Vg-gjBa9YcwSYWFVt0rw9w',
        },
      ]),
      attributes([
        { name: 'Type', value: 'Dark' },
        { name: 'Clothes', value: 'Purple Shirt' },
        { name: 'Ears', value: 'None' },
      ]),
    ],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Links,
        values: [
          {
            name: 'metadata',
            uri: 'https://arweave.net/ebBV1qEYt65AKmM2J5wH_Vg-gjBa9YcwSYWFVt0rw9w',
          },
        ],
      },
      {
        type: ExtensionType.Attributes,
        values: [
          { name: 'Type', value: 'Dark' },
          { name: 'Clothes', value: 'Purple Shirt' },
          { name: 'Ears', value: 'None' },
        ],
      },
    ],
  });

  // When we remove the first extension from the asset.
  await remove(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    recipient: umi.identity.publicKey,
    extensionType: ExtensionType.Attributes,
  }).sendAndConfirm(umi);

  // Then the first extension is removed from the asset.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Links,
        values: [
          {
            name: 'metadata',
            uri: 'https://arweave.net/ebBV1qEYt65AKmM2J5wH_Vg-gjBa9YcwSYWFVt0rw9w',
          },
        ],
      },
    ],
  });
});

test('it cannot remove a manager extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new managed asset.
  // And we initialize an asset with a managed extension.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Managed Asset',
    payer: umi.identity,
    standard: Standard.Managed,
    extensions: [manager(umi.identity.publicKey, DelegateRole.Transfer)],
  }).sendAndConfirm(umi);

  // When we try to remove the manager extension.
  const promise = remove(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    recipient: umi.identity.publicKey,
    extensionType: ExtensionType.Manager,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Extension data invalid/,
  });
});

test('it cannot remove the grouping extension from a non-empty group', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();

  const group = generateSigner(umi);
  // And we create a new group asset.
  await create(umi, {
    asset: group,
    owner: umi.identity.publicKey,
    name: 'Digital Asset',
    payer: umi.identity,
    extensions: [grouping(10)],
  }).sendAndConfirm(umi);

  const asset = generateSigner(umi);
  // And we add an asset to the group.
  await create(umi, {
    asset,
    payer: umi.identity,
    name: 'Asset',
    group: group.publicKey,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, group.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 1n,
        maxSize: 10n,
      },
    ],
  });

  // When we try to remove the group extension.
  const promise = remove(umi, {
    asset: group.publicKey,
    authority: umi.identity,
    recipient: umi.identity.publicKey,
    extensionType: ExtensionType.Grouping,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Group is not empty/,
  });
});

test('it can remove the grouping extension from an empty group', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new asset without extensions.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
    payer: umi.identity,
    extensions: [grouping(10)],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Grouping,
        size: 0n,
        maxSize: 10n,
      },
    ],
  });

  // When we remove the group extension from the asset.
  await remove(umi, {
    asset: asset.publicKey,
    authority: umi.identity,
    recipient: umi.identity.publicKey,
    extensionType: ExtensionType.Grouping,
  }).sendAndConfirm(umi);

  // Then the extension is removed from the asset.
  t.assert((await fetchAsset(umi, asset.publicKey)).extensions.length === 0);
});

test('it cannot remove a non-existing extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we create a new asset with an extension
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
    payer: umi.identity,
    extensions: [
      attributes([
        { name: 'Type', value: 'Dark' },
        { name: 'Clothes', value: 'Purple Shirt' },
        { name: 'Ears', value: 'None' },
      ]),
      metadata({
        symbol: 'MAD',
        uri: 'https://madlads.s3.us-west-2.amazonaws.com/json/8420.json',
      }),
      royalties(500),
    ],
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        values: [
          { name: 'Type', value: 'Dark' },
          { name: 'Clothes', value: 'Purple Shirt' },
          { name: 'Ears', value: 'None' },
        ],
      },
      {
        type: ExtensionType.Royalties,
        basisPoints: 500n,
        constraint: empty(),
      },
      {
        type: ExtensionType.Metadata,
        symbol: 'MAD',
        uri: 'https://madlads.s3.us-west-2.amazonaws.com/json/8420.json',
      },
    ],
  });

  // When we try to remove a non-existing extension.
  const promise = remove(umi, {
    asset: asset.publicKey,
    recipient: umi.identity.publicKey,
    extensionType: ExtensionType.Links,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /Extension not found/,
  });
});
