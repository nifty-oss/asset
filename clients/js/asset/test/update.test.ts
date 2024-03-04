import { getSplSystemProgramId } from '@metaplex-foundation/mpl-toolbox';
import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  ExtensionType,
  attributes,
  blob,
  create,
  fetchAsset,
  getExtensionSerializerFromType,
  initialize,
  links,
  update,
  updateWithBuffer,
} from '../src';
import { createUmi } from './_setup';

test('it can update the name of an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset v1',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    name: 'Digital Asset v1',
  });

  // When we update the name of the asset.
  await update(umi, {
    asset: asset.publicKey,
    name: 'Digital Asset v2',
  }).sendAndConfirm(umi);

  // Then the asset has the new name.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    name: 'Digital Asset v2',
  });
});

test('it can update an asset to be immutable', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);

  // And a new asset.
  const asset = generateSigner(umi);
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset v1',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    mutable: true,
  });

  // When we update the mutability of the asset.
  await update(umi, {
    asset: asset.publicKey,
    mutable: false,
  }).sendAndConfirm(umi);

  // Then the asset is immutable.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    mutable: false,
  });
});

test('it cannot update an immutable asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);

  // And a new asset.
  const asset = generateSigner(umi);
  await create(umi, {
    asset,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Digital Asset v1',
    mutable: false,
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    mutable: false,
  });

  // When we try to update an immutable asset.
  const promise = update(umi, {
    asset: asset.publicKey,
    name: 'Digital Asset v2',
  }).sendAndConfirm(umi);

  // Then we get an error.
  await t.throwsAsync(promise, {
    message: /Immutable asset/,
  });
});

test('it can update the extension of an asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes([
      { traitType: 'Type', value: 'Dark' },
      { traitType: 'Clothes', value: 'Purple Shirt' },
      { traitType: 'Ears', value: 'None' },
    ]),
  }).sendAndConfirm(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [
          { traitType: 'Type', value: 'Dark' },
          { traitType: 'Clothes', value: 'Purple Shirt' },
          { traitType: 'Ears', value: 'None' },
        ],
      },
    ],
  });

  // When we update the extension of the asset.
  const data = getExtensionSerializerFromType(
    ExtensionType.Attributes
  ).serialize(attributes([{ traitType: 'Clothes', value: 'Purple Shirt' }]));
  await update(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    extension: {
      extensionType: ExtensionType.Attributes,
      length: data.length,
      data,
    },
  }).sendAndConfirm(umi);

  // Then the extension is updated.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [{ traitType: 'Clothes', value: 'Purple Shirt' }],
      },
    ],
  });
});

test('it can update the extension of an asset with multiple extensions', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an attributes extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes([
      { traitType: 'Type', value: 'Dark' },
      { traitType: 'Clothes', value: 'Purple Shirt' },
      { traitType: 'Ears', value: 'None' },
    ]),
  }).sendAndConfirm(umi);

  // And we initialize a links extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: links([
      {
        name: 'metadata',
        uri: 'https://arweave.net/ebBV1qEYt65AKmM2J5wH_Vg-gjBa9YcwSYWFVt0rw9w',
      },
    ]),
  }).sendAndConfirm(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [
          { traitType: 'Type', value: 'Dark' },
          { traitType: 'Clothes', value: 'Purple Shirt' },
          { traitType: 'Ears', value: 'None' },
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

  // When we update the extension of the asset.
  const data = getExtensionSerializerFromType(
    ExtensionType.Attributes
  ).serialize(attributes([{ traitType: 'Clothes', value: 'Purple Shirt' }]));
  await update(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    extension: {
      extensionType: ExtensionType.Attributes,
      length: data.length,
      data,
    },
  }).sendAndConfirm(umi);

  // Then the extension is updated.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [{ traitType: 'Clothes', value: 'Purple Shirt' }],
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

test('it can extend the length of an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: attributes([{ traitType: 'Type', value: 'Dark' }]),
  }).sendAndConfirm(umi);

  // And we create a new asset.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [{ traitType: 'Type', value: 'Dark' }],
      },
    ],
  });

  // When we update the extension of the asset.
  const data = getExtensionSerializerFromType(
    ExtensionType.Attributes
  ).serialize(
    attributes([
      { traitType: 'Type', value: 'Dark' },
      { traitType: 'Clothes', value: 'Purple Shirt' },
      { traitType: 'Ears', value: 'None' },
    ])
  );
  await update(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    systemProgram: getSplSystemProgramId(umi),
    extension: {
      extensionType: ExtensionType.Attributes,
      length: data.length,
      data,
    },
  }).sendAndConfirm(umi);

  // Then the extension is updated.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    extensions: [
      {
        type: ExtensionType.Attributes,
        traits: [
          { traitType: 'Type', value: 'Dark' },
          { traitType: 'Clothes', value: 'Purple Shirt' },
          { traitType: 'Ears', value: 'None' },
        ],
      },
    ],
  });
});

test('it can update an asset with a buffer', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const owner = generateSigner(umi);

  // And we initialize an asset with a blob (image) extension.
  let response = await fetch(
    'https://arweave.net/Y8MBS8tqo9XJ_Z1l9V6BIMvhknWxhzP0UxSNBk1OXSs'
  );
  let image = new Uint8Array(await response.arrayBuffer());
  let contentType = response.headers.get('content-type') ?? 'image/png';

  // And we initialize an image extension.
  await initialize(umi, {
    asset,
    payer: umi.identity,
    extension: blob(contentType, image),
  }).sendAndConfirm(umi);

  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');

  // And we create the asset with a blob (image) extension.
  await create(umi, {
    asset,
    owner: owner.publicKey,
    name: 'Blob Asset',
  }).sendAndConfirm(umi);

  let assetAccount = await fetchAsset(umi, asset.publicKey);
  let extension = assetAccount.extensions[0];
  t.true(extension.type === ExtensionType.Blob);

  if (extension.type === ExtensionType.Blob) {
    t.is(extension.contentType, contentType);
    t.is(extension.data.length, image.length);
    t.deepEqual(extension.data, Array.from(image));
  }

  // When we update the asset with a buffer.
  response = await fetch(
    'https://arweave.net/jq15kbD89BMQyc1YIH7PD5RWfFqnxRhVzjt0UZNgDu8'
  );
  image = new Uint8Array(await response.arrayBuffer());
  contentType = response.headers.get('content-type') ?? 'image/png';

  await updateWithBuffer(umi, {
    asset: asset.publicKey,
    payer: umi.identity,
    extension: blob(contentType, image),
  }).sendAndConfirm(umi);

  // Then the asset was updated correctly.
  assetAccount = await fetchAsset(umi, asset.publicKey);
  [extension] = assetAccount.extensions;
  t.true(extension.type === ExtensionType.Blob);

  if (extension.type === ExtensionType.Blob) {
    t.is(extension.contentType, contentType);
    t.is(extension.data.length, image.length);
    t.deepEqual(extension.data, Array.from(image));
  }
});
