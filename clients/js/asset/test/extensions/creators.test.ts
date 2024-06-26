import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  create,
  creators,
  fetchAsset,
  initialize,
  update,
  verify,
} from '../../src';
import { createUmi } from '../_setup';

test('it can create a new asset with a creator', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);
  const creator = generateSigner(umi).publicKey;

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([{ address: creator, share: 100 }]),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Asset with creators',
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
        type: ExtensionType.Creators,
        values: [
          {
            address: creator,
            verified: false,
            share: 100,
          },
        ],
      },
    ],
  });
});

test('it can create a new asset with the authority as a verified creator', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);
  const authority = umi.identity;

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([{ address: authority.publicKey, share: 100 }]),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    authority,
    name: 'Asset with creators',
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
        type: ExtensionType.Creators,
        values: [
          {
            address: authority.publicKey,
            verified: true,
            share: 100,
          },
        ],
      },
    ],
  });
});

test('it can create a new asset with multiple creators', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  const addresses = new Array(10)
    .fill(0)
    .map(() => ({ address: generateSigner(umi).publicKey, share: 10 }));

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators(addresses),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  const expectedCreators = addresses.map((address) => ({
    address: address.address,
    verified: false,
    share: address.share,
  }));

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
        type: ExtensionType.Creators,
        values: expectedCreators,
      },
    ],
  });
});

test("it cannot create an asset with invalid creators' total share", async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);

  // And a list of creators with a total share of 99.
  const addresses = new Array(9)
    .fill(0)
    .map(() => ({ address: generateSigner(umi).publicKey, share: 10 }));

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators(addresses),
  }).sendAndConfirm(umi);

  const promise = create(umi, {
    asset,
    payer: umi.identity,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Extension data invalid/ });
});

test('it maintain a creator verified status on update', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And two creators.
  const creator1 = generateSigner(umi);
  const creator2 = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([
      { address: creator1.publicKey, share: 50 },
      { address: creator2.publicKey, share: 50 },
    ]),
  }).sendAndConfirm(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  // Creator is unverified at this point.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        values: [
          {
            address: creator1.publicKey,
            verified: false,
            share: 50,
          },
          {
            address: creator2.publicKey,
            verified: false,
            share: 50,
          },
        ],
      },
    ],
  });

  // And we verify the creator 1.
  await verify(umi, {
    asset: asset.publicKey,
    creator: creator1,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        values: [
          {
            address: creator1.publicKey,
            verified: true,
            share: 50,
          },
          {
            address: creator2.publicKey,
            verified: false,
            share: 50,
          },
        ],
      },
    ],
  });

  // When we update the extension of the asset removing/adding a new creator.
  const creator3 = generateSigner(umi);
  await update(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    extension: creators([
      { address: creator1.publicKey, share: 50 },
      { address: creator3.publicKey, share: 50 },
    ]),
  }).sendAndConfirm(umi);

  // Then the creator 1 should remain verified.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        values: [
          {
            address: creator1.publicKey,
            verified: true,
            share: 50,
          },
          {
            address: creator3.publicKey,
            verified: false,
            share: 50,
          },
        ],
      },
    ],
  });
});

test('it cannot remove a verified creator on update', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And two creators.
  const creator1 = generateSigner(umi);
  const creator2 = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([
      { address: creator1.publicKey, share: 50 },
      { address: creator2.publicKey, share: 50 },
    ]),
  }).sendAndConfirm(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  // And we verify the creator 1.
  await verify(umi, {
    asset: asset.publicKey,
    creator: creator1,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        values: [
          {
            address: creator1.publicKey,
            verified: true,
            share: 50,
          },
          {
            address: creator2.publicKey,
            verified: false,
            share: 50,
          },
        ],
      },
    ],
  });

  // When we try to update the extension removing a verified creator.
  const promise = update(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    extension: creators([{ address: creator2.publicKey, share: 100 }]),
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Extension data invalid/ });
});

test('it can remove an unverified creator on update', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And two creators.
  const creator1 = generateSigner(umi);
  const creator2 = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([
      { address: creator1.publicKey, share: 50 },
      { address: creator2.publicKey, share: 50 },
    ]),
  }).sendAndConfirm(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        values: [
          {
            address: creator1.publicKey,
            verified: false,
            share: 50,
          },
          {
            address: creator2.publicKey,
            verified: false,
            share: 50,
          },
        ],
      },
    ],
  });

  // When we update the extension removing an unverified creator.
  await update(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    extension: creators([{ address: creator2.publicKey, share: 100 }]),
  }).sendAndConfirm(umi);

  // Then the unverified creator is removed
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        values: [
          {
            address: creator2.publicKey,
            verified: false,
            share: 100,
          },
        ],
      },
    ],
  });
});

test('it cannot update creators with invalid total share', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And two creators.
  const creator1 = generateSigner(umi);
  const creator2 = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([
      { address: creator1.publicKey, share: 50 },
      { address: creator2.publicKey, share: 50 },
    ]),
  }).sendAndConfirm(umi);

  // And we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Creators,
        values: [
          {
            address: creator1.publicKey,
            verified: false,
            share: 50,
          },
          {
            address: creator2.publicKey,
            verified: false,
            share: 50,
          },
        ],
      },
    ],
  });

  // When we trye tp update the extension with an invalid total share.
  const promise = update(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    extension: creators([{ address: creator2.publicKey, share: 90 }]),
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Extension data invalid/ });
});
