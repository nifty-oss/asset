import { generateSigner } from '@metaplex-foundation/umi';
import { httpDownloader } from '@metaplex-foundation/umi-downloader-http';
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
} from '../../src';
import { createUmi } from '../_setup';

test('it can create a new asset with a creator', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // And we initialize an asset with a creators extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators([{ address: umi.identity.publicKey, share: 100 }]),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: [
          {
            address: umi.identity.publicKey,
            verified: false,
            share: 100,
            padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
          },
        ],
      },
    ],
  });
});

test('it can create a new asset with multiple creators', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

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
    holder: holder.publicKey,
    name: 'Asset with creators',
  }).sendAndConfirm(umi);

  const expectedCreators = addresses.map((address) => ({
    address: address.address,
    verified: false,
    share: address.share,
    padding: new Uint8Array([0, 0, 0, 0, 0, 0]),
  }));

  // Then an asset was created with the correct data.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Creators,
        creators: expectedCreators,
      },
    ],
  });
});

test("it cannot create an asset with invalid creators' total share", async (t) => {
  // Given a Umi instance and a new signer.
  const umi = (await createUmi()).use(httpDownloader());
  const asset = generateSigner(umi);

  // And a list of creators with a total share of 99.
  const addresses = new Array(9)
    .fill(0)
    .map(() => ({ address: generateSigner(umi).publicKey, share: 10 }));

  // And we try to initialize an asset with a creators extension.
  const promise = initialize(umi, {
    asset,
    payer: umi.identity,
    extension: creators(addresses),
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /Extension data invalid/ });
});
