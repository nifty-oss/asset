/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  ExtensionInput,
  ExtensionInputArgs,
  getExtensionInputSerializer,
} from '../types';

// Accounts.
export type AllocateInstructionAccounts = {
  /** Asset account */
  asset: Signer;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type AllocateInstructionData = {
  discriminator: number;
  extensionInput: ExtensionInput;
};

export type AllocateInstructionDataArgs = {
  extensionInput: ExtensionInputArgs;
};

export function getAllocateInstructionDataSerializer(): Serializer<
  AllocateInstructionDataArgs,
  AllocateInstructionData
> {
  return mapSerializer<
    AllocateInstructionDataArgs,
    any,
    AllocateInstructionData
  >(
    struct<AllocateInstructionData>(
      [
        ['discriminator', u8()],
        ['extensionInput', getExtensionInputSerializer()],
      ],
      { description: 'AllocateInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 4 })
  ) as Serializer<AllocateInstructionDataArgs, AllocateInstructionData>;
}

// Args.
export type AllocateInstructionArgs = AllocateInstructionDataArgs;

// Instruction.
export function allocate(
  context: Pick<Context, 'programs'>,
  input: AllocateInstructionAccounts & AllocateInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'asset',
    'AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'
  );

  // Accounts.
  const resolvedAccounts = {
    asset: {
      index: 0,
      isWritable: true as boolean,
      value: input.asset ?? null,
    },
    payer: {
      index: 1,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: AllocateInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.systemProgram.value) {
    if (resolvedAccounts.payer.value) {
      resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
        'systemProgram',
        '11111111111111111111111111111111'
      );
      resolvedAccounts.systemProgram.isWritable = false;
    }
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
  const data = getAllocateInstructionDataSerializer().serialize(
    resolvedArgs as AllocateInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
