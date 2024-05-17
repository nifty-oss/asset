import {
  Context,
  PublicKey,
  TransactionBuilderGroup,
  generateSigner,
  none,
  transactionBuilderGroup,
} from '@metaplex-foundation/umi';
import { ASSET_PROGRAM_ID, SYSTEM_PROGRAM_ID, close } from '.';
import { TypedExtension, getExtensionSerializerFromType } from './extensions';
import { allocate } from './generated/instructions/allocate';
import {
  UpdateInstructionAccounts,
  UpdateInstructionArgs,
  update,
} from './generated/instructions/update';
import { DEFAULT_CHUNK_SIZE, write } from './write';

export function updateWithBuffer(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: UpdateInstructionAccounts &
    Omit<UpdateInstructionArgs, 'extension'> & {
      extension: TypedExtension;
      chunkSize?: number;
    } & { proxy?: PublicKey }
): TransactionBuilderGroup {
  const data = getExtensionSerializerFromType(input.extension.type).serialize(
    input.extension
  );
  const chunked = data.length > (input.chunkSize ?? DEFAULT_CHUNK_SIZE);

  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  if (chunked) {
    const buffer = generateSigner(context);

    let updateIx = update(context, {
      ...input,
      buffer: buffer.publicKey,
      systemProgram: SYSTEM_PROGRAM_ID,
      extension: none(),
    });

    if (input.proxy) {
      updateIx = updateIx.addRemainingAccounts({
        pubkey: ASSET_PROGRAM_ID,
        isWritable: false,
        isSigner: false,
      });
    }

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
      .append(updateIx)
      .append(
        close(context, {
          buffer,
          recipient: input.payer?.publicKey ?? context.payer.publicKey,
        })
      );
  }

  let updateIx = update(context, {
    ...input,
    extension: {
      extensionType: input.extension.type,
      length: data.length,
      data,
    },
  });

  if (input.proxy) {
    updateIx = updateIx.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return transactionBuilderGroup([updateIx]);
}
