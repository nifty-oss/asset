import {
  Context,
  TransactionBuilderGroup,
  some,
  transactionBuilderGroup,
} from '@metaplex-foundation/umi';
import { TypedExtension, getExtensionSerializerFromType } from './extensions';
import { DEFAULT_CHUNK_SIZE, write } from './write';
import {
  AllocateInstructionAccounts,
  allocate,
} from './generated/instructions/allocate';

export function initialize(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: AllocateInstructionAccounts & { extension: TypedExtension }
): TransactionBuilderGroup {
  const data = getExtensionSerializerFromType(input.extension.type).serialize(
    input.extension
  );

  const chunked = data.length > DEFAULT_CHUNK_SIZE;

  const builder = allocate(context, {
    ...input,
    extension: {
      extensionType: input.extension.type,
      length: data.length,
      data: chunked ? null : some(data),
    },
  });

  if (chunked) {
    return write(context, {
      ...input,
      data,
    })
      .prepend(builder)
      .sequential();
  }

  return transactionBuilderGroup([builder]);
}
