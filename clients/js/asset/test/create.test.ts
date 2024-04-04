import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  attributes,
  create,
  creators,
  empty,
  fetchAsset,
  grouping,
  initialize,
  links,
  metadata,
  royalties,
} from '../src';
import { createUmi } from './_setup';

test('it can create an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

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
});

test('it can create a new asset with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes([{ traitType: 'head', value: 'hat' }]),
  }).sendAndConfirm(umi);

  // When we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [
          {
            traitType: 'head',
            value: 'hat',
          },
        ],
      },
    ],
  });
});

test('it can create a new asset with multiple extensions', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // When we create a new asset with multiple extensions.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    extensions: [
      attributes([
        { traitType: 'Attributes Count', value: '2' },
        { traitType: 'Type', value: 'Dark' },
        { traitType: 'Clothes', value: 'Purple Shirt' },
        { traitType: 'Ears', value: 'None' },
        { traitType: 'Mouth', value: 'None' },
        { traitType: 'Eyes', value: 'None' },
        { traitType: 'Hat', value: 'Blue Cap' },
      ]),
      links([
        {
          name: 'metadata',
          uri: 'https://arweave.net/ebBV1qEYt65AKmM2J5wH_Vg-gjBa9YcwSYWFVt0rw9w',
        },
      ]),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [
          { traitType: 'Attributes Count', value: '2' },
          { traitType: 'Type', value: 'Dark' },
          { traitType: 'Clothes', value: 'Purple Shirt' },
          { traitType: 'Ears', value: 'None' },
          { traitType: 'Mouth', value: 'None' },
          { traitType: 'Eyes', value: 'None' },
          { traitType: 'Hat', value: 'Blue Cap' },
        ],
      },
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

test('it can create a soulbound asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Soulbound Asset',
    standard: Standard.Soulbound,
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Soulbound,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
    name: 'Soulbound Asset',
  });
});

test('it can create an asset with a group', async (t) => {
  // Given a Umi instance and an authority signer.
  const umi = await createUmi();
  const authority = generateSigner(umi);

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await initialize(umi, {
    asset: groupAsset,
    payer: umi.identity,
    extension: grouping(10),
  }).sendAndConfirm(umi);

  await create(umi, {
    asset: groupAsset,
    authority: authority.publicKey,
    name: 'Group',
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

  // When we create an asset with a group with the same authority.
  const asset = generateSigner(umi);
  await create(umi, {
    asset,
    payer: umi.identity,
    authority,
    name: 'Asset',
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  // Then the group is set on the asset.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    name: 'Asset',
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

test('it cannot set a group on create with wrong authority', async (t) => {
  // Given a Umi instance and an authority signer.
  const umi = await createUmi();
  const authority = generateSigner(umi);

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await initialize(umi, {
    asset: groupAsset,
    payer: umi.identity,
    extension: grouping(10),
  }).sendAndConfirm(umi);

  await create(umi, {
    asset: groupAsset,
    authority: authority.publicKey,
    name: 'Group',
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

  // When we create an asset with a group using the wrong authority.
  const asset = generateSigner(umi);
  const promise = create(umi, {
    asset,
    payer: umi.identity,
    name: 'Asset',
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Invalid authority/ });
});

test('it cannot set a group on create with authority not a signer', async (t) => {
  // Given a Umi instance and an authority signer.
  const umi = await createUmi();
  const authority = generateSigner(umi);

  // And we create a group asset.
  const groupAsset = generateSigner(umi);
  await initialize(umi, {
    asset: groupAsset,
    payer: umi.identity,
    extension: grouping(10),
  }).sendAndConfirm(umi);

  await create(umi, {
    asset: groupAsset,
    authority: authority.publicKey,
    name: 'Group',
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

  // When we create an asset with a group without the authority as a signer.
  const asset = generateSigner(umi);
  const promise = create(umi, {
    asset,
    authority: authority.publicKey,
    payer: umi.identity,
    name: 'Asset',
    group: groupAsset.publicKey,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /missing required signature/ });
});

test('it can create an asset with a collection', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const authority = generateSigner(umi);

  // And we create a group asset.
  const group = generateSigner(umi);
  await create(umi, {
    asset: group,
    payer: umi.identity,
    name: 'Mad Lads',
    authority,
    extensions: [grouping(10000)],
  }).sendAndConfirm(umi);

  // When we create a new asset with multiple extensions.
  await create(umi, {
    asset,
    owner: umi.identity.publicKey,
    payer: umi.identity,
    authority,
    name: 'Mad Lads #8420',
    group: group.publicKey,
    extensions: [
      metadata({
        symbol: 'MAD',
        uri: 'https://madlads.s3.us-west-2.amazonaws.com/json/8420.json',
      }),
      royalties(500),
      creators([
        {
          address: publicKey('5XvhfmRjwXkGp3jHGmaKpqeerNYjkuZZBYLVQYdeVcRv'),
          share: 0,
        },
        {
          address: publicKey('2RtGg6fsFiiF1EQzHqbd66AhW7R5bWeQGpTbv2UMkCdW'),
          share: 100,
        },
      ]),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  //
  // (the order of the extension is different since the program uses
  // a swap remove procedure when processing the extensions)
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: umi.identity.publicKey,
    authority: authority.publicKey,
    group: group.publicKey,
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'MAD',
        uri: 'https://madlads.s3.us-west-2.amazonaws.com/json/8420.json',
      },
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: publicKey('5XvhfmRjwXkGp3jHGmaKpqeerNYjkuZZBYLVQYdeVcRv'),
            share: 0,
          },
          {
            address: publicKey('2RtGg6fsFiiF1EQzHqbd66AhW7R5bWeQGpTbv2UMkCdW'),
            share: 100,
          },
        ],
      },
      { type: ExtensionType.Royalties, basisPoints: 500n, constraint: empty() },
    ],
  });
});
