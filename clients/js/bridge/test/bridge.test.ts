import {
  generateSigner,
  percentAmount,
  publicKey,
} from '@metaplex-foundation/umi';
import test from 'ava';
import {
  Discriminator,
  State,
  Vault,
  bridge,
  create,
  fetchVault,
  findBridgeAssetPda,
  findVaultPda,
} from '../src';
import { createNft, createUmi } from './_setup';
import { Asset, fetchAsset } from '@nifty-oss/asset';
import {
  TokenStandard,
  createProgrammableNft,
  fetchDigitalAssetWithToken,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  TokenState,
  findAssociatedTokenPda,
} from '@metaplex-foundation/mpl-toolbox';

test('it can bridge an asset to a token (NFT)', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata non-fungible.
  const owner = generateSigner(umi);
  const mint = await createNft(umi, {
    name: 'Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    tokenOwner: owner.publicKey,
  });

  // And we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // And we bridge the asset.
  await bridge(umi, {
    mint: mint.publicKey,
    owner,
  }).sendAndConfirm(umi);

  // And we check that the token was bridged.
  let asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: mint.publicKey })
  );

  t.like(asset, <Asset>{
    holder: owner.publicKey,
  });

  let legacy = await fetchDigitalAssetWithToken(
    umi,
    mint.publicKey,
    publicKey(
      findAssociatedTokenPda(umi, {
        mint: mint.publicKey,
        owner: publicKey(findVaultPda(umi, { mint: mint.publicKey })),
      })
    )
  );

  t.true(legacy.token.state === TokenState.Initialized);
  t.true(legacy.token.amount === 1n);

  // When we bridge the asset back to token.
  await bridge(umi, {
    mint: mint.publicKey,
    owner,
  }).sendAndConfirm(umi);

  // Then the bridge vault is in the correct state.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
    mint: mint.publicKey,
  });

  // And the asset is transferred back to the vault.
  asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: mint.publicKey })
  );

  t.like(asset, <Asset>{
    holder: vault.publicKey,
  });

  // And the non-fungible is held by the owner.
  legacy = await fetchDigitalAssetWithToken(
    umi,
    mint.publicKey,
    publicKey(
      findAssociatedTokenPda(umi, {
        mint: mint.publicKey,
        owner: owner.publicKey,
      })
    )
  );

  t.true(legacy.token.state === TokenState.Initialized);
  t.true(legacy.token.owner === owner.publicKey);
  t.true(legacy.token.amount === 1n);
});

test('it can bridge a token (NFT) to an asset', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();

  // And a Token Metadata non-fungible.
  const owner = generateSigner(umi);
  const mint = await createNft(umi, {
    name: 'Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    tokenOwner: owner.publicKey,
  });

  // And we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // When we bridge the asset.
  await bridge(umi, {
    mint: mint.publicKey,
    owner,
  }).sendAndConfirm(umi);

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Active,
    mint: mint.publicKey,
  });

  // And the asset is transferred to the owner.
  const asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: mint.publicKey })
  );

  t.like(asset, <Asset>{
    holder: owner.publicKey,
  });

  // And the non-fungible is held by the bridge.
  const legacy = await fetchDigitalAssetWithToken(
    umi,
    mint.publicKey,
    publicKey(
      findAssociatedTokenPda(umi, {
        mint: mint.publicKey,
        owner: vault.publicKey,
      })
    )
  );

  t.true(legacy.token.state === TokenState.Initialized);
  t.true(legacy.token.owner === vault.publicKey);
  t.true(legacy.token.amount === 1n);
});

test('it can bridge an asset to a token (pNFT)', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();
  const mint = generateSigner(umi);

  // And a Token Metadata non-fungible.
  const owner = generateSigner(umi);
  await createProgrammableNft(umi, {
    name: 'pNFT Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    tokenOwner: owner.publicKey,
    mint,
  }).sendAndConfirm(umi);

  // And we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // And we bridge the asset.
  await bridge(umi, {
    mint: mint.publicKey,
    owner,
    tokenStandard: TokenStandard.ProgrammableNonFungible,
  }).sendAndConfirm(umi);

  // And we check that the token was bridged.
  let asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: mint.publicKey })
  );

  t.like(asset, <Asset>{
    holder: owner.publicKey,
  });

  let legacy = await fetchDigitalAssetWithToken(
    umi,
    mint.publicKey,
    publicKey(
      findAssociatedTokenPda(umi, {
        mint: mint.publicKey,
        owner: publicKey(findVaultPda(umi, { mint: mint.publicKey })),
      })
    )
  );

  t.true(legacy.token.state === TokenState.Frozen);
  t.true(legacy.token.amount === 1n);

  // When we bridge the asset back to token.
  await bridge(umi, {
    mint: mint.publicKey,
    owner,
    tokenStandard: TokenStandard.ProgrammableNonFungible,
  }).sendAndConfirm(umi);

  // Then the bridge vault is in the correct state.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Idle,
    mint: mint.publicKey,
  });

  // And the asset is transferred back to the vault.
  asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: mint.publicKey })
  );

  t.like(asset, <Asset>{
    holder: vault.publicKey,
  });

  // And the non-fungible is held by the owner.
  legacy = await fetchDigitalAssetWithToken(
    umi,
    mint.publicKey,
    publicKey(
      findAssociatedTokenPda(umi, {
        mint: mint.publicKey,
        owner: owner.publicKey,
      })
    )
  );

  t.true(legacy.token.state === TokenState.Frozen);
  t.true(legacy.token.owner === owner.publicKey);
  t.true(legacy.token.amount === 1n);
});

test('it can bridge a token (pNFT) to an asset', async (t) => {
  // Given a Umi instance.
  const umi = await createUmi();
  const mint = generateSigner(umi);

  // And a Token Metadata programmable non-fungible.
  const owner = generateSigner(umi);
  await createProgrammableNft(umi, {
    name: 'Bridge Asset',
    uri: 'https://asset.bridge',
    sellerFeeBasisPoints: percentAmount(5.5),
    tokenOwner: owner.publicKey,
    mint,
  }).sendAndConfirm(umi);

  // And we create the asset on the bridge.
  await create(umi, {
    mint: mint.publicKey,
    updateAuthority: umi.identity,
  }).sendAndConfirm(umi);

  // When we bridge the asset.
  await bridge(umi, {
    mint: mint.publicKey,
    owner,
    tokenStandard: TokenStandard.ProgrammableNonFungible,
  }).sendAndConfirm(umi);

  // Then the bridge vault is created.
  const vault = await fetchVault(
    umi,
    findVaultPda(umi, { mint: mint.publicKey })
  );

  t.like(vault, <Vault>{
    discriminator: Discriminator.Vault,
    state: State.Active,
    mint: mint.publicKey,
  });

  // And the asset is transferred to the owner.
  const asset = await fetchAsset(
    umi,
    findBridgeAssetPda(umi, { mint: mint.publicKey })
  );

  t.like(asset, <Asset>{
    holder: owner.publicKey,
  });

  // And the non-fungible is held by the bridge.
  const legacy = await fetchDigitalAssetWithToken(
    umi,
    mint.publicKey,
    publicKey(
      findAssociatedTokenPda(umi, {
        mint: mint.publicKey,
        owner: vault.publicKey,
      })
    )
  );

  t.true(legacy.token.state === TokenState.Frozen);
  t.true(legacy.token.owner === vault.publicKey);
  t.true(legacy.token.amount === 1n);
});
