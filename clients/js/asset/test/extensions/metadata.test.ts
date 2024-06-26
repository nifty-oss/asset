import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  create,
  fetchAsset,
  initialize,
  metadata,
} from '../../src';
import { createUmi } from '../_setup';

test('it can create a new asset with a metadata', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize a metadata extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: metadata({
      symbol: 'SMB',
      description: 'A metadata extension',
      uri: 'https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc',
      imageUri:
        'https://arweave.net/Va823FYwx0jqbbtpGjJryJCr2FtJwfRTM1f8nNn3dyg',
    }),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // When we create the asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Metadata Asset',
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
        type: ExtensionType.Metadata,
        symbol: 'SMB',
        description: 'A metadata extension',
        uri: 'https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc',
        imageUri:
          'https://arweave.net/Va823FYwx0jqbbtpGjJryJCr2FtJwfRTM1f8nNn3dyg',
      },
    ],
  });
});
