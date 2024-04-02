import { generateSigner } from '@metaplex-foundation/umi';
import {
  Discriminator,
  Standard,
  State,
  fetchAsset,
  transfer,
} from '@nifty-oss/asset';
import test from 'ava';
import { PROXY_PROGRAM_ID, create } from '../src';
import { findProxiedAssetPda } from '../src/pda';
import { createUmi } from './_setup';

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
  }).sendAndConfirm(umi);

  const asset = findProxiedAssetPda(umi, { stub: stub.publicKey });
  const recipient = generateSigner(umi).publicKey;

  // When we transfer the proxied asset through the proxy program.
  await transfer(umi, {
    asset,
    signer: owner,
    recipient,
    proxy: PROXY_PROGRAM_ID,
  }).sendAndConfirm(umi);

  // Then the asset is transferred.
  t.like(await fetchAsset(umi, asset), {
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.Proxied,
    owner: recipient,
  });
});
