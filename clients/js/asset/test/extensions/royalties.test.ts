import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import test from 'ava';
import { royalties } from '../../src/extensions/royalties';
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
  or,
  ownedBy,
  pubkeyMatch,
} from '../../src';
import { createUmi } from '../_setup';
import { OperatorType } from '../../src/extensions/operatorType';

test('it can mint a new asset a royalties extension with a PubkeyMatch constraint', async (t) => {
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

test('it can mint a new asset with royalties extension with a OwnedBy constraint', async (t) => {
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

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy(Account.Asset, [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const notConstraint = not(ownedByConstraint);

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

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy(Account.Asset, [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const pubkeyMatchConstraint = pubkeyMatch(Account.Asset, [
    '8UWRNwLHxD5DmEJ2cjVFdVpCNhfxL7bLkYpXG1o9srEN',
  ]);
  const andConstraint = and([ownedByConstraint, pubkeyMatchConstraint]);

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

test('it can mint a new asset with a royalties extension with an OR constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy(Account.Asset, [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const pubkeyMatchConstraint = pubkeyMatch(Account.Asset, [
    '8UWRNwLHxD5DmEJ2cjVFdVpCNhfxL7bLkYpXG1o9srEN',
  ]);
  const orConstraint = or([ownedByConstraint, pubkeyMatchConstraint]);

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
        constraint: orConstraint,
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
        constraint: orConstraint,
      },
    ],
  }));
});

test('it can mint a new asset with a nested royalties extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy(Account.Asset, [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const pubkeyMatchConstraint = pubkeyMatch(Account.Asset, [
    '8UWRNwLHxD5DmEJ2cjVFdVpCNhfxL7bLkYpXG1o9srEN',
  ]);
  const orConstraint = or([ownedByConstraint, pubkeyMatchConstraint]);
  const notConstraint = not(orConstraint);

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
