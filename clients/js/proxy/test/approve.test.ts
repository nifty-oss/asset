import { generateSigner } from '@metaplex-foundation/umi';
import {
  Asset,
  DelegateRole,
  ExtensionType,
  approve,
  delegateInput,
  fetchAsset,
  getExtension,
} from '@nifty-oss/asset';
import test from 'ava';
import { findProxiedAssetPda } from '../src';
import { create } from '../src';
import { createUmi } from './_setup';

test('it can set a delegate through the proxy program', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const owner = generateSigner(umi);

  // And a stub signer.
  const stub = generateSigner(umi);

  // When we create a new proxied asset.
  await create(umi, {
    stub,
    owner: owner.publicKey,
    payer: umi.identity,
    name: 'Proxied Asset',
  }).sendAndConfirm(umi);

  const [address] = findProxiedAssetPda(umi, { stub: stub.publicKey });
  let asset = await fetchAsset(umi, address);
  const proxy = getExtension(asset, ExtensionType.Proxy);

  // When we approve a delegate.
  const delegate = generateSigner(umi).publicKey;
  await approve(umi, {
    asset: address,
    owner,
    delegate,
    delegateInput: delegateInput('Some', { roles: [DelegateRole.Transfer] }),
    proxy: proxy?.program,
  }).sendAndConfirm(umi);

  // Then the delegate is set.
  t.like(await fetchAsset(umi, address), <Asset>{
    delegate: {
      address: delegate,
      roles: [DelegateRole.Transfer],
    },
  });
});
