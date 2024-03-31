import {
  Context,
  OptionOrNullable,
  TransactionBuilder,
  none,
} from '@metaplex-foundation/umi';
import { TypedExtension, getExtensionSerializerFromType } from './extensions';
import { ExtensionInputArgs } from './generated';
import {
  CreateInstructionAccounts,
  CreateInstructionArgs,
  create as baseCreate,
} from './generated/instructions/create';

export function create(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: CreateInstructionAccounts &
    Omit<CreateInstructionArgs, 'extensions'> & {
      extensions?: TypedExtension[];
    }
): TransactionBuilder {
  let extensions: OptionOrNullable<Array<ExtensionInputArgs>> = none();

  if (input.extensions) {
    extensions = input.extensions.map((extension) => {
      const data = getExtensionSerializerFromType(extension.type).serialize(
        extension
      );
      return {
        extensionType: extension.type,
        length: data.length,
        data,
      };
    });
  }

  return baseCreate(context, {
    ...input,
    extensions,
  });
}
