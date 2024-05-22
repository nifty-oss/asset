import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  Type,
  create,
  fetchAsset,
  getProperty,
  initialize,
  properties,
} from '../../src';
import { createUmi } from '../_setup';

test('it can create a new asset with properties', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an asset with properties.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: properties([
      { name: 'name', value: 'nifty' },
      { name: 'version', value: 1n },
      { name: 'alpha', value: false },
    ]),
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
        type: ExtensionType.Properties,
        values: [
          {
            type: Type.Text,
            name: 'name',
            value: 'nifty',
          },
          {
            type: Type.Number,
            name: 'version',
            value: 1n,
          },
          {
            type: Type.Boolean,
            name: 'alpha',
            value: false,
          },
        ],
      },
    ],
  });

  const name = getProperty(assetAccount, 'name', Type.Text);
  if (name) {
    t.deepEqual(name.value, 'nifty');
  }

  const version = getProperty(assetAccount, 'version', Type.Number);
  if (version) {
    t.deepEqual(version.value, 1n);
  }

  const alpha = getProperty(assetAccount, 'alpha', Type.Boolean);
  if (alpha) {
    t.deepEqual(alpha.value, false);
  }
});
