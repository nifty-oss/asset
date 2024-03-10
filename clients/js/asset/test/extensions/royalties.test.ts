import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Constraint,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  and,
  fetchAsset,
  mint,
  not,
  or,
  ownedBy,
  pubkeyMatch,
} from '../../src';
import { royalties } from '../../src/extensions/royalties';
import { createUmi } from '../_setup';

test('it can mint a new asset a royalties extension with a PubkeyMatch constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  const basisPoints = BigInt(500);

  // When we create a new asset.
  await mint(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [
      royalties(
        basisPoints,
        pubkeyMatch('Asset', [
          publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
        ])
      ),
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
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: {
          type: 'PubkeyMatch',
          account: 'Asset',
          pubkeys: [publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8')],
        },
      },
    ],
  });
});

test('it can mint a new asset with royalties extension with a OwnedBy constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  const basisPoints = BigInt(500);

  // When we create a new asset.
  await mint(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [
      royalties(
        basisPoints,
        ownedBy('Asset', [
          publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
        ])
      ),
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
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: {
          type: 'OwnedBy',
          account: 'Asset',
          owners: [publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8')],
        },
      },
    ],
  });
});

test('it can mint a new asset with a royalties extension with NOT constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy('Asset', [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const notConstraint = not(ownedByConstraint);

  // When we create a new asset.
  await mint(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [royalties(basisPoints, notConstraint)],
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
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: notConstraint,
      },
    ],
  });
});

test('it can mint a new asset with a royalties extension with an AND constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy('Asset', [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const pubkeyMatchConstraint = pubkeyMatch('Asset', [
    '8UWRNwLHxD5DmEJ2cjVFdVpCNhfxL7bLkYpXG1o9srEN',
  ]);
  const andConstraint = and([ownedByConstraint, pubkeyMatchConstraint]);

  // When we create a new asset.
  await mint(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [royalties(basisPoints, andConstraint)],
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
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: andConstraint,
      },
    ],
  });
});

test('it can mint a new asset with a royalties extension with an OR constraint', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  const basisPoints = BigInt(500);

  const ownedByConstraint = ownedBy('Asset', [
    publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
  ]);
  const pubkeyMatchConstraint = pubkeyMatch('Asset', [
    '8UWRNwLHxD5DmEJ2cjVFdVpCNhfxL7bLkYpXG1o9srEN',
  ]);
  const orConstraint = or([ownedByConstraint, pubkeyMatchConstraint]);

  // When we create a new asset.
  await mint(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [royalties(basisPoints, orConstraint)],
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
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: orConstraint,
      },
    ],
  });
});

test('it can mint a new asset with a nested royalties extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  const basisPoints = BigInt(500);

  const constraint: Constraint = {
    type: 'And',
    constraints: [
      {
        type: 'OwnedBy',
        account: 'Asset',
        owners: [
          publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
          publicKey('BbSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
          publicKey('CcSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
        ],
      },
      {
        type: 'Or',
        constraints: [
          {
            type: 'PubkeyMatch',
            account: 'Asset',
            pubkeys: [
              publicKey('CcSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
            ],
          },
          {
            type: 'Not',
            constraint: {
              type: 'Not',
              constraint: {
                type: 'Not',
                constraint: {
                  type: 'PubkeyMatch',
                  account: 'Asset',
                  pubkeys: [
                    publicKey('8UWRNwLHxD5DmEJ2cjVFdVpCNhfxL7bLkYpXG1o9srEN'),
                  ],
                },
              },
            },
          },
        ],
      },
    ],
  };

  // When we create a new asset.
  await mint(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [royalties(basisPoints, constraint)],
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
        type: ExtensionType.Royalties,
        basisPoints,
        constraint,
      },
    ],
  });
});
