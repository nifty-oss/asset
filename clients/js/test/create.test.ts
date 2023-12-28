import { generateSigner, some } from '@metaplex-foundation/umi';
import test from 'ava';
import { Asset, ExtensionType, create, fetchAsset, findAssetPda, getAttributesSerializer, initialize } from '../src';
import { createUmi } from './_setup';

test('it can create a account', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const holder = generateSigner(umi);
  const mold = generateSigner(umi);

  // When we create a new account.
  await create(umi, {
    holder: holder.publicKey,
    mold,
    name: 'Digital Asset',
    symbol: 'DA',
  }).sendAndConfirm(umi);

  // Then an account was created with the correct data.
  t.like(await fetchAsset(umi, findAssetPda(umi, { mold: mold.publicKey })), <
    Asset
  >{
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
  });
});

test.only('it can create a new account with an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const holder = generateSigner(umi);
  const mold = generateSigner(umi);

  // And we initialize an extension.
  await initialize(umi, {
    mold,
    extensionType: ExtensionType.Attributes,
    data: some(
      getAttributesSerializer().serialize({
        traits: [{ traitType: 'head', value: 'hat' }],
      })
    ),
  }).sendAndConfirm(umi);

  // When we create a new account.
  await create(umi, {
    holder: holder.publicKey,
    mold,
    name: 'Digital Asset',
    symbol: 'DA',
  }).sendAndConfirm(umi);

  // Then an account was created with the correct data.
  t.like(await fetchAsset(umi, findAssetPda(umi, { mold: mold.publicKey })), <
    Asset
  >{
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
  });
});
