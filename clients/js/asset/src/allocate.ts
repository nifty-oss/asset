import { Context, TransactionBuilder, some } from '@metaplex-foundation/umi';
import { TypedExtension, getExtensionSerializerFromType } from './extensions';
import {
  AllocateInstructionAccounts,
  allocate as baseAllocate,
} from './generated/instructions/allocate';

export function allocate(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: AllocateInstructionAccounts & { extension: TypedExtension }
): TransactionBuilder {
  const data = getExtensionSerializerFromType(input.extension.type).serialize(
    input.extension
  );

  return baseAllocate(context, {
    ...input,
    extension: {
      extensionType: input.extension.type,
      length: data.length,
      data: some(data),
    },
  });
}
