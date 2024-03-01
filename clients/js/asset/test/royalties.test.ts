import {
  TokenStandard,
  createProgrammableNft,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  generateSigner,
  percentAmount,
  publicKey,
} from '@metaplex-foundation/umi';
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import {
  string,
  publicKey as publicKeySerializer,
} from '@metaplex-foundation/umi/serializers';
import { mplToolbox } from '@metaplex-foundation/mpl-toolbox';
import { PublicKey } from '@solana/web3.js';
import test from 'ava';
import {
  Vault,
  create,
  fetchVault,
  findVaultPda,
  niftyBridge,
  Discriminator as BridgeDiscriminator,
  State as BridgeState,
  BRIDGE_PROGRAM_ID,
  bridge,
} from '@nifty-oss/bridge';
import {
  Account,
  Asset,
  ExtensionType,
  fetchAsset,
  getExtensionSerializerFromType,
  niftyAsset,
  not,
  pubkeyMatch,
  royalties,
  update,
} from '../src';

const createUmi = async () =>
  (await basecreateUmi())
    .use(mplToolbox())
    .use(niftyBridge())
    .use(niftyAsset());

test('pubkeymatch failing blocks a transfer', async (t) => {
  const umi = await createUmi();
  const mintSigner = generateSigner(umi);
  const owner = generateSigner(umi);
  const notOwner = generateSigner(umi);

  const basisPoints = BigInt(550);

  // And a Token Metadata programmable non-fungible.
  await createProgrammableNft(umi, {
    name: 'pNFT Bridge Asset',
    uri: 'https://asset.bridge',
    symbol: 'BA',
    sellerFeeBasisPoints: percentAmount(5.5),
    tokenOwner: owner.publicKey,
    mint: mintSigner,
  }).sendAndConfirm(umi);

  // When we create the asset on the bridge.
  await create(umi, {
    mint: mintSigner.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // And the asset is created.
  const asset = umi.eddsa.findPda(BRIDGE_PROGRAM_ID, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(mintSigner.publicKey),
  ]);

  const pubkeyMatchConstraint = pubkeyMatch(Account.Asset, [
    publicKey(PublicKey.default),
  ]);
  const defaultConstraint = not(pubkeyMatchConstraint);

  t.like(await fetchAsset(umi, asset), <Asset>{
    extensions: [
      {
        type: ExtensionType.Metadata,
        symbol: 'BA',
        uri: 'https://asset.bridge',
      },
      {
        type: ExtensionType.Royalties,
        basisPoints,
        constraint: defaultConstraint,
      },
    ],
  });

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mintSigner.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: BridgeDiscriminator.Vault,
    state: BridgeState.Idle,
    mint: mintSigner.publicKey,
  });

  // We create a PubkeyMatch constraint that will block the transfer to the owner.
  const constraint = pubkeyMatch(Account.Recipient, [publicKey(notOwner)]);

  // We update the default royalties extension.
  const data = getExtensionSerializerFromType(
    ExtensionType.Royalties
  ).serialize(royalties({ basisPoints, constraint }));
  await update(umi, {
    asset: asset[0],
    payer: umi.identity,
    extension: {
      extensionType: ExtensionType.Royalties,
      length: data.length,
      data,
    },
  }).sendAndConfirm(umi);

  // Bridging the asset should fail.
  const promise = bridge(umi, {
    mint: mintSigner.publicKey,
    owner,
    tokenStandard: TokenStandard.ProgrammableNonFungible,
  }).sendAndConfirm(umi);

  await t.throwsAsync(promise, {
    message: /Assertion Failure/,
  });
});
