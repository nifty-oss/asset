import {
  createSignerFromKeypair,
  generateSigner,
  publicKey,
} from '@metaplex-foundation/umi';
import { fromWeb3JsKeypair } from '@metaplex-foundation/umi-web3js-adapters';
import {
  Discriminator,
  ExtensionType,
  Standard,
  State,
  fetchAsset,
  getExtension,
  transfer,
} from '@nifty-oss/asset';
import { Keypair } from '@solana/web3.js';
import test from 'ava';
import { create } from '../src';
import { findProxiedAssetPda } from '../src/pda';
import { STUB_KEY, createUmi } from './_setup';

test('it cannot transfer a non-signer proxied asset', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);

  // And a stub signer.
  const stub = generateSigner(umi);

  // And we create a new proxied asset.
  await create(umi, {
    stub,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Proxied Asset',
  }).sendAndConfirm(umi);

  const asset = findProxiedAssetPda(umi, { stub: stub.publicKey });
  const recipient = generateSigner(umi).publicKey;

  // When we try to transfer the proxied asset as a non-signer (not
  // throuhg the proxy program).
  const promise = transfer(umi, {
    asset,
    signer: owner,
    recipient,
  }).sendAndConfirm(umi);

  // Then we expect an error.
  await t.throwsAsync(promise, {
    message: /missing required signature/,
  });
});

test('it can transfer a proxied asset through the proxy program', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);

  // And a stub signer.
  const stub = generateSigner(umi);

  // And we create a new proxied asset.
  await create(umi, {
    stub,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Proxied Asset',
  }).sendAndConfirm(umi);

  const asset = findProxiedAssetPda(umi, { stub: stub.publicKey });
  const proxy = getExtension(await fetchAsset(umi, asset), ExtensionType.Proxy);
  const recipient = generateSigner(umi).publicKey;

  // When we transfer the proxied asset through the proxy program.
  await transfer(umi, {
    asset,
    signer: owner,
    recipient,
    proxy: proxy?.program,
  }).sendAndConfirm(umi);

  // Then the asset is transferred.
  t.like(await fetchAsset(umi, asset), {
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Proxied,
    owner: recipient,
  });
});

test('it can execute custom logic on transfer', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);

  // And a stub signer.
  const stub = generateSigner(umi);

  // And we create a new proxied asset.
  await create(umi, {
    stub,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Proxied Asset',
  }).sendAndConfirm(umi);

  const address = findProxiedAssetPda(umi, { stub: stub.publicKey });
  let asset = await fetchAsset(umi, address);

  let attributes = getExtension(asset, ExtensionType.Attributes);
  // initial value for the number of transfers
  const initial = attributes?.traits[0].value;
  t.true(parseInt(initial!) === 0);

  // And we transfer the proxied asset through the proxy program.
  const recipient = generateSigner(umi).publicKey;
  const proxy = getExtension(asset, ExtensionType.Proxy);

  await transfer(umi, {
    asset: address,
    signer: owner,
    recipient,
    proxy: proxy?.program,
  }).sendAndConfirm(umi);

  // Then the asset is transferred.
  asset = await fetchAsset(umi, address);
  t.like(asset, {
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Proxied,
    owner: recipient,
  });

  // And the custom logic is executed.
  attributes = getExtension(asset, ExtensionType.Attributes);
  const current = parseInt(attributes?.traits[0].value!);
  t.true(current === 1);
  t.assert(parseInt(initial!) < current);
});

test('it can transfer the proxy asset multiple times', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();

  // And a (fixed) stub signer.
  const stub = createSignerFromKeypair(
    umi,
    fromWeb3JsKeypair(Keypair.fromSecretKey(STUB_KEY))
  );
  const address = findProxiedAssetPda(umi, { stub: stub.publicKey });

  if (!(await umi.rpc.accountExists(publicKey(address)))) {
    // we create a new proxied asset if needed.
    await create(umi, {
      stub,
      owner: stub.publicKey,
      payer: umi.identity,
      name: 'Proxied Asset',
    }).sendAndConfirm(umi);
  }

  // And we transfer the proxied asset through the proxy program.
  const recipient = generateSigner(umi);
  let asset = await fetchAsset(umi, address);

  let attributes = getExtension(asset, ExtensionType.Attributes);
  // initial value for the number of transfers
  const initial = attributes?.traits[0].value;

  const proxy = getExtension(asset, ExtensionType.Proxy);

  await transfer(umi, {
    asset: address,
    signer: stub,
    recipient: recipient.publicKey,
    proxy: proxy?.program,
  }).sendAndConfirm(umi);

  // When transfer it back to the fixed signer.
  await transfer(umi, {
    asset: address,
    signer: recipient,
    recipient: stub.publicKey,
    proxy: proxy?.program,
  }).sendAndConfirm(umi);

  // Then the custom logic is executed.
  attributes = getExtension(
    await fetchAsset(umi, address),
    ExtensionType.Attributes
  );
  const current = parseInt(attributes?.traits[0].value!);
  t.assert(parseInt(initial!) < current);
});
