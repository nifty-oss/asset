import { generateSigner } from '@metaplex-foundation/umi';
import test from 'ava';
import { attributes } from '../src';
import { allocate } from '../src/allocate';
import { createUmi } from './_setup';

test('it can allocate an extension', async (t) => {
  // Given a Umi instance and a new signer.
  const umi = await createUmi();
  const asset = generateSigner(umi);

  // When we allocate an extension.
  await allocate(umi, {
    asset,
    payer: umi.identity,
    extension: attributes([{ name: 'head', value: 'hat' }]),
  }).sendAndConfirm(umi);

  // Then the account was created
  t.true(await umi.rpc.accountExists(asset.publicKey));
});
