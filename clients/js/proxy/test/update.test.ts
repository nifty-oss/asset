import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { createUmi } from './_setup';
import { create } from '../src';
import {
  Discriminator,
  ExtensionType,
  Standard,
  State,
  fetchAsset,
  getExtension,
  update,
} from '@nifty-oss/asset';
import { findProxiedAssetPda } from '../src/pda';

test('it can update the name of a proxied asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);
  const authority = generateSigner(umi);

  // And a stub signer.
  const stub = generateSigner(umi);

  // And we create a new proxied asset.
  await create(umi, {
    stub,
    owner: owner.publicKey,
    authority,
    payer: umi.identity,
    name: 'Proxied Asset',
  }).sendAndConfirm(umi);

  let asset = await fetchAsset(
    umi,
    findProxiedAssetPda(umi, { stub: stub.publicKey })
  );
  t.like(asset, {
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Proxied,
    name: 'Proxied Asset',
  });

  const proxy = getExtension(asset, ExtensionType.Proxy);
  t.like(proxy, {
    authority: authority.publicKey,
  });

  // When we update the name of the proxied asset.
  await update(umi, {
    asset: asset.publicKey,
    authority,
    proxy: proxy?.program,
    name: 'Updated Proxied Asset',
  }).sendAndConfirm(umi);

  // Then the asset should have the updated name.
  asset = await fetchAsset(umi, asset.publicKey);
  t.like(asset, {
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Proxied,
    name: 'Updated Proxied Asset',
  });
});

test('it cannot update a proxied asset without the proxy authority', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);
  const authority = generateSigner(umi);

  // And a stub signer.
  const stub = generateSigner(umi);

  // And we create a new proxied asset.
  await create(umi, {
    stub,
    owner: owner.publicKey,
    authority,
    payer: umi.identity,
    name: 'Proxied Asset',
  }).sendAndConfirm(umi);

  let asset = await fetchAsset(
    umi,
    findProxiedAssetPda(umi, { stub: stub.publicKey })
  );
  t.like(asset, {
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Proxied,
    name: 'Proxied Asset',
  });

  const proxy = getExtension(asset, ExtensionType.Proxy);
  t.like(proxy, {
    authority: authority.publicKey,
  });

  // When we try to update the name with a 'fake' authority.
  const fake = generateSigner(umi);
  const promise = update(umi, {
    asset: asset.publicKey,
    authority: fake,
    proxy: proxy?.program,
    name: 'Updated Proxied Asset',
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, { message: /invalid program argument/ });
});
