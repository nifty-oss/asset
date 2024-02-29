import { generateSigner, publicKey } from '@metaplex-foundation/umi';
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import { mplToolbox } from '@metaplex-foundation/mpl-toolbox';
import test from 'ava';
import { niftyBridge } from '@nifty-oss/bridge';
import { royalties } from '../src/extensions/royalties';
import {
  Account,
  Asset,
  Discriminator,
  ExtensionType,
  OperatorType,
  Standard,
  State,
  fetchAsset,
  mint,
  niftyAsset,
  pubkeyMatch,
} from '../src';

const createUmi = async () =>
  (await basecreateUmi())
    .use(mplToolbox())
    .use(niftyBridge())
    .use(niftyAsset());

test('pubkeymatch failing blocks a transfer', async (t) => {
  const umi = await createUmi();
  const asset = generateSigner(umi);
  const holder = generateSigner(umi);

  const basisPoints = BigInt(1000);

  // Mint an asset with a pubkey match royalty extension.
  await mint(umi, {
    asset,
    holder: holder.publicKey,
    payer: umi.identity,
    name: 'Digital Asset',
    mutable: true,
    standard: Standard.NonFungible,
    extensions: [
      royalties({
        basisPoints,
        constraint: pubkeyMatch(Account.Asset, [
          publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8'),
        ]),
      }),
    ],
  }).sendAndConfirm(umi);

  // Then an asset was created with the correct data.
  t.like(await fetchAsset(umi, asset.publicKey), <Asset>(<unknown>{
    discriminator: Discriminator.Asset,
    state: State.Unlocked,
    standard: Standard.NonFungible,
    holder: holder.publicKey,
    authority: umi.identity.publicKey,
    extensions: [
      {
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: {
          type: OperatorType.PubkeyMatch,
          account: Account.Asset,
          pubkeys: [publicKey('AaSZHtdnHTcW4En23vJfmXxhZceoAfZnAjc8kYvherJ8')],
        },
      },
    ],
  }));
});
