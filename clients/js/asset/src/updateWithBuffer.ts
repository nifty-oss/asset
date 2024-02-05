import {
  Context,
  TransactionBuilderGroup,
  generateSigner,
  transactionBuilderGroup,
} from '@metaplex-foundation/umi';
import { getSplSystemProgramId } from '@metaplex-foundation/mpl-toolbox';
import { TypedExtension, getExtensionSerializerFromType } from './extensions';
import {
  UpdateInstructionAccounts,
  UpdateInstructionArgs,
  allocate,
  update,
} from './generated';
import { DEFAULT_CHUNK_SIZE, write } from './write';

export function updateWithBuffer(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: UpdateInstructionAccounts &
    Omit<UpdateInstructionArgs, 'extension'> & { extension: TypedExtension }
): TransactionBuilderGroup {
  const data = getExtensionSerializerFromType(input.extension.type).serialize(
    input.extension
  );
  const chunked = data.length > DEFAULT_CHUNK_SIZE;

  if (chunked) {
    const buffer = generateSigner(context);

    return write(context, {
      ...input,
      asset: buffer,
      data,
    })
      .prepend(
        allocate(context, {
          ...input,
          asset: buffer,
          extension: {
            extensionType: input.extension.type,
            length: data.length,
            data: null,
          },
        })
      )
      .append(
        update(context, {
          ...input,
          buffer: buffer.publicKey,
          systemProgram: getSplSystemProgramId(context),
          extension: {
            extensionType: input.extension.type,
            length: data.length,
            data: chunked ? null : data,
          },
        })
      );
  }

  return transactionBuilderGroup([
    update(context, {
      ...input,
      extension: {
        extensionType: input.extension.type,
        length: data.length,
        data,
      },
    }),
  ]);
}
