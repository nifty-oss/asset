/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  none,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  bool,
  mapSerializer,
  option,
  string,
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
  Standard,
  StandardArgs,
  getExtensionInputSerializer,
  getStandardSerializer,
} from '../types';

// Accounts.
export type CreateInstructionAccounts = {
  /** Asset account */
  asset: Signer;
  /** The authority of the asset */
  authority?: PublicKey | Pda | Signer;
  /** The owner of the asset */
  owner?: PublicKey | Pda;
  /** Asset account of the group */
  group?: PublicKey | Pda;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CreateInstructionData = {
  discriminator: number;
  name: string;
  standard: Standard;
  mutable: boolean;
  extensions: Option<Array<ExtensionInput>>;
};

export type CreateInstructionDataArgs = {
  name: string;
  standard?: StandardArgs;
  mutable?: boolean;
  extensions?: OptionOrNullable<Array<ExtensionInputArgs>>;
};

export function getCreateInstructionDataSerializer(): Serializer<
  CreateInstructionDataArgs,
  CreateInstructionData
> {
  return mapSerializer<CreateInstructionDataArgs, any, CreateInstructionData>(
    struct<CreateInstructionData>(
      [
        ['discriminator', u8()],
        ['name', string()],
        ['standard', getStandardSerializer()],
        ['mutable', bool()],
        ['extensions', option(array(getExtensionInputSerializer()))],
      ],
      { description: 'CreateInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: 2,
      standard: value.standard ?? Standard.NonFungible,
      mutable: value.mutable ?? true,
      extensions: value.extensions ?? none(),
    })
  ) as Serializer<CreateInstructionDataArgs, CreateInstructionData>;
}

// Args.
export type CreateInstructionArgs = CreateInstructionDataArgs;

// Instruction.
export function create(
  context: Pick<Context, 'identity' | 'programs'>,
  input: CreateInstructionAccounts & CreateInstructionArgs
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
    authority: {
      index: 1,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    owner: {
      index: 2,
      isWritable: false as boolean,
      value: input.owner ?? null,
    },
    group: {
      index: 3,
      isWritable: true as boolean,
      value: input.group ?? null,
    },
    payer: {
      index: 4,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 5,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity;
  }
  if (!resolvedAccounts.owner.value) {
    resolvedAccounts.owner.value = context.identity.publicKey;
  }
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
  const data = getCreateInstructionDataSerializer().serialize(
    resolvedArgs as CreateInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
