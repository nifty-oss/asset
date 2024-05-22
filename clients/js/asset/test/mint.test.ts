import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  attributes,
  blob,
  creators,
  fetchAsset,
  links,
  metadata,
  mint,
  royalties,
} from '../src';
import { createUmi } from './_setup';

test('it can mint an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // When we create a new asset.
  await mint(umi, {
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

test('it can mint an asset with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // When we mint a new asset with an extension.
  await mint(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    extensions: [attributes([{ name: 'head', value: 'hat' }])],
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
        values: [
          {
            name: 'head',
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
  const owner = generateSigner(umi);

  // And an image.
  const response = await fetch(
    'https://arweave.net/wGChHSDTXTP9oAtTaYh9s8j1MRE0IPmYtH5greqWwZ4'
  );
  const image = new Uint8Array(await response.arrayBuffer());
  const contentType = response.headers.get('content-type') ?? 'image/png';

  // When we create a new asset with multiple extensions.
  await mint(umi, {
    asset,
    authority: publicKey('mdaoxg4DVGptU4WSpzGyVpK3zqsgn7Qzx5XNgWTcEA2'),
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'SMB #1355 (test)',
    extensions: [
      attributes([
        { name: 'Attributes Count', value: '2' },
        { name: 'Type', value: 'Skeleton' },
        { name: 'Clothes', value: 'Orange Jacket' },
        { name: 'Ears', value: 'None' },
        { name: 'Mouth', value: 'None' },
        { name: 'Eyes', value: 'None' },
        { name: 'Hat', value: 'Crown' },
      ]),
      creators([
        {
          address: publicKey('mdaoxg4DVGptU4WSpzGyVpK3zqsgn7Qzx5XNgWTcEA2'),
          share: 0,
        },
        {
          address: publicKey('HAryckvjyViFQEmhmMoCtqqBMJnpXEYViamyDhZUJfnG'),
          share: 100,
        },
        {
          address: publicKey('9uBX3ASjxWvNBAD1xjbVaKA74mWGZys3RGSF7DdeDD3F'),
          share: 0,
        },
      ]),
      links([
        {
          name: 'external_url',
          uri: 'https://solanamonkey.business/',
        },
      ]),
      blob(contentType, image),
      metadata({
        symbol: 'SMB',
        description:
          'SMB is a collection of 5000 randomly generated 24x24 pixels NFTs on the Solana Blockchain.\
          Each SolanaMonkey is unique and comes with different type and attributes varying in rarity.',
      }),
      royalties(500),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    owner: owner.publicKey,
  });
});
