import {
  Context,
  OptionOrNullable,
  PublicKey,
  TransactionBuilder,
  none,
} from '@metaplex-foundation/umi';
import { TypedExtension, getExtensionSerializerFromType } from './extensions';
import { ASSET_PROGRAM_ID, ExtensionInputArgs } from './generated';
import {
  UpdateInstructionAccounts,
  UpdateInstructionArgs,
  update as baseUpdate,
} from './generated/instructions/update';

export function update(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: UpdateInstructionAccounts &
    Omit<UpdateInstructionArgs, 'extension'> & {
      extension?: TypedExtension;
    } & { proxy?: PublicKey }
): TransactionBuilder {
  let extension: OptionOrNullable<ExtensionInputArgs> = none();

  if (input.extension) {
    const data = getExtensionSerializerFromType(input.extension.type).serialize(
      input.extension
    );
    extension = {
      extensionType: input.extension.type,
      length: data.length,
      data,
    };
  }

  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseUpdate(context, {
    ...input,
    extension,
  });

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
