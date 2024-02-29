import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import test from 'ava';
import { royalties } from '../src/extensions/royalties';
import {
  Account,
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  and,
  attributes,
  fetchAsset,
  links,
  mint,
  not,
  ownedBy,
  pubkeyMatch,
} from '../src';
import { createUmi } from './_setup';
import { OperatorType } from '../src/extensions/operatorType';

test('it can mint an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // When we create a new asset.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
  });
});

test('it can mint an asset with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // When we mint a new asset with an extension.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    extensions: [attributes([{ traitType: 'head', value: 'hat' }])],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
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

test('it can mint a new asset with multiple extensions', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // When we create a new asset.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
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
    holder: holder.publicKey,
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

test('it can mint a new asset with a PubkeyMatch royalties extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  const basisPoints = BigInt(500);

  // When we create a new asset.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [
      royalties({
        basisPoints,
        constraint: pubkeyMatch(Account.Asset, [
          publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
        ]),
      }),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>(<unknown>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: {
          type: OperatorType.PubkeyMatch,
          account: Account.Asset,
          pubkeys: [publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8')],
        },
      },
    ],
  }));
});

test('it can mint a new asset with a OwnedBy royalties extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  const basisPoints = BigInt(500);

  // When we create a new asset.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [
      royalties({
        basisPoints,
        constraint: ownedBy(Account.Asset, [
          publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
        ]),
      }),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>(<unknown>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: {
          type: OperatorType.OwnedBy,
          account: Account.Asset,
          owners: [publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8')],
        },
      },
    ],
  }));
});

test('it can mint a new asset with a royalties extension with NOT constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  console.log(asset.publicKey.toString());

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy(Account.Asset, [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const notConstraint = not(ownedByConstraint);

  console.log('notConstraint', notConstraint);

  // When we create a new asset.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [
      royalties({
        basisPoints,
        constraint: notConstraint,
      }),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>(<unknown>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: notConstraint,
      },
    ],
  }));
});

test('it can mint a new asset with a royalties extension with an AND constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  console.log(asset.publicKey.toString());

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy(Account.Asset, [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const pubkeyMatchConstraint = pubkeyMatch(Account.Asset, [
    '8UWRNwLHxD5DmEJ2cjVFdVpCNhfxL7bLkYpXG1o9srEN',
  ]);
  const andConstraint = and([
    ownedByConstraint,
    pubkeyMatchConstraint,
    ownedByConstraint,
    pubkeyMatchConstraint,
  ]);

  console.log('andConstraint', andConstraint);

  // When we create a new asset.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [
      royalties({
        basisPoints,
        constraint: andConstraint,
      }),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>(<unknown>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: andConstraint,
      },
    ],
  }));
});
