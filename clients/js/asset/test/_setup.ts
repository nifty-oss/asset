/* eslint-disable import/no-extraneous-dependencies */
import {
  createNft as baseCreateNft,
  createProgrammableNft as baseCreateProgrammableNft,
  findMetadataPda,
  verifyCollectionV1,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  PublicKey,
  Signer,
  Umi,
  generateSigner,
  publicKey,
  some,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import { niftyAsset } from '../src';

export const createUmi = async () => (await basecreateUmi()).use(niftyAsset());

export const createNft = async (
  umi: Umi,
  input: Omit<
    Parameters<typeof baseCreateNft>[1],
    'mint' | 'amount' | 'tokenStandard'
  > & { mint?: Signer }
): Promise<Signer> => {
  input.mint = input.mint ?? generateSigner(umi);
  await baseCreateNft(umi, {
    ...input,
    mint: input.mint,
  }).sendAndConfirm(umi);

  return input.mint;
};

export const createProgrammableNft = async (
  umi: Umi,
  input: Omit<
    Parameters<typeof baseCreateNft>[1],
    'mint' | 'amount' | 'tokenStandard'
  > & { mint?: Signer }
): Promise<Signer> => {
  input.mint = input.mint ?? generateSigner(umi);
  await baseCreateProgrammableNft(umi, {
    ...input,
    mint: input.mint,
  }).sendAndConfirm(umi);

  return input.mint;
};

export const createCollectionNft = async (
  umi: Umi,
  input: Omit<
    Parameters<typeof baseCreateNft>[1],
    'mint' | 'amount' | 'tokenStandard'
  > & { mint?: Signer }
): Promise<Signer> => createNft(umi, { ...input, isCollection: true });

export const createVerifiedNft = async (
  umi: Umi,
  input: Omit<
    Parameters<typeof baseCreateNft>[1],
    'mint' | 'amount' | 'tokenStandard'
  > & {
    mint?: Signer;
    collectionMint: PublicKey;
    collectionAuthority?: Signer;
  }
): Promise<Signer> => {
  const { collectionMint, collectionAuthority = umi.identity, ...rest } = input;
  const mint = await createNft(umi, {
    ...rest,
    collection: some({ verified: false, key: collectionMint }),
  });
  const effectiveMint = publicKey(rest.mint ?? mint.publicKey);

  await transactionBuilder()
    .add(
      verifyCollectionV1(umi, {
        authority: collectionAuthority,
        collectionMint,
        metadata: findMetadataPda(umi, { mint: effectiveMint })[0],
      })
    )
    .sendAndConfirm(umi);

  return mint;
};
