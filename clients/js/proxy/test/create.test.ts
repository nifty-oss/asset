import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { createUmi } from './_setup';
import { create } from '../src';
import { Discriminator, Standard, State, fetchAsset } from '@nifty-oss/asset';
import { findProxiedAssetPda } from '../src/pda';

test('it can create a proxied asset', async (t) => {
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
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(
    await fetchAsset(umi, findProxiedAssetPda(umi, { stub: stub.publicKey })),
    {
      discriminator: Discriminator.Asset,
      state: State.Unlocked,
      standard: Standard.Proxied,
    }
  );
});
