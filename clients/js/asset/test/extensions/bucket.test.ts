import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  bucket,
  create,
  fetchAsset,
  initialize,
} from '../../src';
import { createUmi } from '../_setup';

test('it can create a new asset with a bucket', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an asset with a bucket extension.
  const response = await fetch(
    'https://arweave.net/Y8MBS8tqo9XJ_Z1l9V6BIMvhknWxhzP0UxSNBk1OXSs'
  );
  const data = new Uint8Array(await response.arrayBuffer());

  // And we initialize an image extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: bucket(data),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Bucket Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  const assetAccount = await fetchAsset(umi, asset.publicKey);
  t.like(assetAccount, <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: owner.publicKey,
    authority: umi.identity.publicKey,
  });

  // And the blob (image) extension was added.
  const extension = assetAccount.extensions[0];
  t.true(extension.type === ExtensionType.Bucket);

  // And the blob (image) has the correct data.
  if (extension.type === ExtensionType.Blob) {
    t.is(extension.data.length, data.length);
    t.deepEqual(extension.data, Array.from(data));
  }
});
