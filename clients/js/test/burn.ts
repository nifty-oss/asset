import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Asset,
  Discriminator,
  Standard,
  State,
  burn,
  create,
  fetchAsset,
} from '../src';
import { createUmi } from './_setup';

test('it can burn an asset as a holder', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
  });

  // Burn the asset.
  await burn(umi, {
    asset: asset.publicKey,
    signer: holder,
  }).sendAndConfirm(umi);

  // Then the asset is gone.
  t.false(await umi.rpc.accountExists(asset.publicKey), 'asset exists');
});

test('invalid signer cannot burn', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);
  const invalid = generateSigner(umi);

  // When we create a new asset.
  await create(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
  });

  // Burn the asset.
  const promise = burn(umi, {
    asset: asset.publicKey,
    signer: invalid,
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, {
    message: /Invalid holder or burn delegate/,
  });

  // Then the asset still exists.
  t.true(await umi.rpc.accountExists(asset.publicKey), 'asset exists');
});
