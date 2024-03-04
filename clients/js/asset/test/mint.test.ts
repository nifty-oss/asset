import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  ExtensionType,
  Standard,
  State,
  attributes,
  fetchAsset,
  links,
  mint,
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
    extensions: [attributes([{ traitType: 'head', value: 'hat' }])],
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

test('it can mint a new asset with multiple extensions', async (t) => {
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
