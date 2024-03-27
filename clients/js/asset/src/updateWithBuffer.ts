import {
  Context,
  TransactionBuilderGroup,
  generateSigner,
  publicKey,
  transactionBuilderGroup,
} from '@metaplex-foundation/umi';
import { TypedExtension, getExtensionSerializerFromType } from './extensions';
import { allocate } from './generated';
import { DEFAULT_CHUNK_SIZE, write } from './write';
import {
  UpdateInstructionAccounts,
  UpdateInstructionArgs,
  update,
} from './generated/instructions/update';
import { SystemProgram } from '@solana/web3.js';

export function updateWithBuffer(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: UpdateInstructionAccounts &
    Omit<UpdateInstructionArgs, 'extension'> & {
      extension: TypedExtension;
      chunkSize?: number;
    }
): TransactionBuilderGroup {
  const data = getExtensionSerializerFromType(input.extension.type).serialize(
    input.extension
  );
  const chunked = data.length > (input.chunkSize ?? DEFAULT_CHUNK_SIZE);

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
          systemProgram: publicKey(SystemProgram.programId.toBase58()),
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
