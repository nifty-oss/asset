/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  TokenStandard,
  TokenStandardArgs,
  findMasterEditionPda,
  findMetadataPda,
  findTokenRecordPda,
} from '@metaplex-foundation/mpl-token-metadata';
import { findAssociatedTokenPda } from '@metaplex-foundation/mpl-toolbox';
import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  publicKey,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { resolveBridgeAsset } from '../../hooked';
import { findVaultPda } from '../accounts';
import {
  PickPartial,
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  expectPublicKey,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type BridgeInstructionAccounts = {
  /** Asset account of the mint (pda of `['nifty::bridge::asset', mint pubkey]`) */
  asset?: PublicKey | Pda;
  /** Bridge account for the asset (pda of `['nifty::bridge::vault', mint pubkey]`) */
  vault?: PublicKey | Pda;
  /** Token owner account */
  owner: Signer;
  /** Token account */
  token?: PublicKey | Pda;
  /** Mint account of the token */
  mint: PublicKey | Pda;
  /** Metadata account of the mint */
  metadata?: PublicKey | Pda;
  /** Master Edition of the mint */
  masterEdition?: PublicKey | Pda;
  /** Owner token record account */
  tokenRecord?: PublicKey | Pda;
  /** Vault token account */
  vaultToken?: PublicKey | Pda;
  /** Vault token record account */
  vaultTokenRecord?: PublicKey | Pda;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** Nifty Asset program */
  niftyAssetProgram?: PublicKey | Pda;
  /** Metaplex Token Metadata program */
  tokenMetadataProgram?: PublicKey | Pda;
  /** System program */
  systemProgram?: PublicKey | Pda;
  /** Instructions sysvar account */
  sysvarInstructions?: PublicKey | Pda;
  /** SPL Token program */
  splTokenProgram?: PublicKey | Pda;
  /** SPL ATA program */
  splAtaProgram?: PublicKey | Pda;
  /** Token Auth Rules program */
  authorizationRulesProgram?: PublicKey | Pda;
  /** Token Auth Rules account */
  authorizationRules?: PublicKey | Pda;
  /** Group asset account */
  groupAsset?: PublicKey | Pda;
};

// Data.
export type BridgeInstructionData = { discriminator: number };

export type BridgeInstructionDataArgs = {};

export function getBridgeInstructionDataSerializer(): Serializer<
  BridgeInstructionDataArgs,
  BridgeInstructionData
> {
  return mapSerializer<BridgeInstructionDataArgs, any, BridgeInstructionData>(
    struct<BridgeInstructionData>([['discriminator', u8()]], {
      description: 'BridgeInstructionData',
    }),
    (value) => ({ ...value, discriminator: 0 })
  ) as Serializer<BridgeInstructionDataArgs, BridgeInstructionData>;
}

// Extra Args.
export type BridgeInstructionExtraArgs = { tokenStandard?: TokenStandardArgs };

// Args.
export type BridgeInstructionArgs = PickPartial<
  BridgeInstructionExtraArgs,
  'tokenStandard'
>;

// Instruction.
export function bridge(
  context: Pick<Context, 'eddsa' | 'identity' | 'payer' | 'programs'>,
  input: BridgeInstructionAccounts & BridgeInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'bridge',
    'BridgezKrNugsZwTcyAMYba643Z93RzC2yN1Y24LwAkm'
  );

  // Accounts.
  const resolvedAccounts = {
    asset: {
      index: 0,
      isWritable: true as boolean,
      value: input.asset ?? null,
    },
    vault: {
      index: 1,
      isWritable: true as boolean,
      value: input.vault ?? null,
    },
    owner: {
      index: 2,
      isWritable: false as boolean,
      value: input.owner ?? null,
    },
    token: {
      index: 3,
      isWritable: true as boolean,
      value: input.token ?? null,
    },
    mint: { index: 4, isWritable: false as boolean, value: input.mint ?? null },
    metadata: {
      index: 5,
      isWritable: true as boolean,
      value: input.metadata ?? null,
    },
    masterEdition: {
      index: 6,
      isWritable: false as boolean,
      value: input.masterEdition ?? null,
    },
    tokenRecord: {
      index: 7,
      isWritable: true as boolean,
      value: input.tokenRecord ?? null,
    },
    vaultToken: {
      index: 8,
      isWritable: true as boolean,
      value: input.vaultToken ?? null,
    },
    vaultTokenRecord: {
      index: 9,
      isWritable: true as boolean,
      value: input.vaultTokenRecord ?? null,
    },
    payer: {
      index: 10,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    niftyAssetProgram: {
      index: 11,
      isWritable: false as boolean,
      value: input.niftyAssetProgram ?? null,
    },
    tokenMetadataProgram: {
      index: 12,
      isWritable: false as boolean,
      value: input.tokenMetadataProgram ?? null,
    },
    systemProgram: {
      index: 13,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    sysvarInstructions: {
      index: 14,
      isWritable: false as boolean,
      value: input.sysvarInstructions ?? null,
    },
    splTokenProgram: {
      index: 15,
      isWritable: false as boolean,
      value: input.splTokenProgram ?? null,
    },
    splAtaProgram: {
      index: 16,
      isWritable: false as boolean,
      value: input.splAtaProgram ?? null,
    },
    authorizationRulesProgram: {
      index: 17,
      isWritable: false as boolean,
      value: input.authorizationRulesProgram ?? null,
    },
    authorizationRules: {
      index: 18,
      isWritable: false as boolean,
      value: input.authorizationRules ?? null,
    },
    groupAsset: {
      index: 19,
      isWritable: false as boolean,
      value: input.groupAsset ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: BridgeInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.asset.value) {
    resolvedAccounts.asset = {
      ...resolvedAccounts.asset,
      ...resolveBridgeAsset(
        context,
        resolvedAccounts,
        resolvedArgs,
        programId,
        true
      ),
    };
  }
  if (!resolvedAccounts.vault.value) {
    resolvedAccounts.vault.value = findVaultPda(context, {
      mint: expectPublicKey(resolvedAccounts.mint.value),
    });
  }
  if (!resolvedAccounts.token.value) {
    resolvedAccounts.token.value = findAssociatedTokenPda(context, {
      mint: expectPublicKey(resolvedAccounts.mint.value),
      owner: expectPublicKey(resolvedAccounts.owner.value),
    });
  }
  if (!resolvedAccounts.metadata.value) {
    resolvedAccounts.metadata.value = findMetadataPda(context, {
      mint: expectPublicKey(resolvedAccounts.mint.value),
    });
  }
  if (!resolvedAccounts.masterEdition.value) {
    resolvedAccounts.masterEdition.value = findMasterEditionPda(context, {
      mint: expectPublicKey(resolvedAccounts.mint.value),
    });
  }
  if (!resolvedAccounts.tokenRecord.value) {
    if (resolvedArgs.tokenStandard === TokenStandard.ProgrammableNonFungible) {
      resolvedAccounts.tokenRecord.value = findTokenRecordPda(context, {
        mint: expectPublicKey(resolvedAccounts.mint.value),
        token: expectPublicKey(resolvedAccounts.token.value),
      });
    }
  }
  if (!resolvedAccounts.vaultToken.value) {
    resolvedAccounts.vaultToken.value = findAssociatedTokenPda(context, {
      mint: expectPublicKey(resolvedAccounts.mint.value),
      owner: expectPublicKey(resolvedAccounts.vault.value),
    });
  }
  if (!resolvedAccounts.vaultTokenRecord.value) {
    if (resolvedArgs.tokenStandard === TokenStandard.ProgrammableNonFungible) {
      resolvedAccounts.vaultTokenRecord.value = findTokenRecordPda(context, {
        mint: expectPublicKey(resolvedAccounts.mint.value),
        token: expectPublicKey(resolvedAccounts.vaultToken.value),
      });
    }
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.niftyAssetProgram.value) {
    resolvedAccounts.niftyAssetProgram.value = context.programs.getPublicKey(
      'niftyAsset',
      'AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'
    );
    resolvedAccounts.niftyAssetProgram.isWritable = false;
  }
  if (!resolvedAccounts.tokenMetadataProgram.value) {
    resolvedAccounts.tokenMetadataProgram.value = context.programs.getPublicKey(
      'mplTokenMetadata',
      'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
    );
    resolvedAccounts.tokenMetadataProgram.isWritable = false;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }
  if (!resolvedAccounts.sysvarInstructions.value) {
    resolvedAccounts.sysvarInstructions.value = publicKey(
      'Sysvar1nstructions1111111111111111111111111'
    );
  }
  if (!resolvedAccounts.splTokenProgram.value) {
    resolvedAccounts.splTokenProgram.value = context.programs.getPublicKey(
      'splToken',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    );
    resolvedAccounts.splTokenProgram.isWritable = false;
  }
  if (!resolvedAccounts.splAtaProgram.value) {
    resolvedAccounts.splAtaProgram.value = context.programs.getPublicKey(
      'splAssociatedToken',
      'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
    );
    resolvedAccounts.splAtaProgram.isWritable = false;
  }
  if (!resolvedAccounts.authorizationRulesProgram.value) {
    if (resolvedArgs.tokenStandard === TokenStandard.ProgrammableNonFungible) {
      resolvedAccounts.authorizationRulesProgram.value =
        context.programs.getPublicKey(
          'mplTokenAuthRules',
          'auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg'
        );
      resolvedAccounts.authorizationRulesProgram.isWritable = false;
    }
  }
  if (!resolvedArgs.tokenStandard) {
    resolvedArgs.tokenStandard = TokenStandard.NonFungible;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getBridgeInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
