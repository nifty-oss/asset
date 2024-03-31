import {
  Context,
  TransactionBuilderGroup,
  transactionBuilderGroup,
} from '@metaplex-foundation/umi';
import { TypedExtension } from './extensions';
import {
  CreateInstructionAccounts,
  CreateInstructionArgs,
  create,
} from './generated/instructions/create';
import { initialize } from './initialize';

export function mint(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: CreateInstructionAccounts &
    Omit<CreateInstructionArgs, 'extensions'> & {
      extensions?: TypedExtension[];
    }
): TransactionBuilderGroup {
  let builder = transactionBuilderGroup();

  if (input.extensions) {
    input.extensions.forEach((extension: TypedExtension) => {
      builder = builder.append(
        initialize(context, {
          ...input,
          extension,
        }).builders
      );
    });
  }
  // drop extensions from input
  const { extensions, ...remaining } = input;
  return builder.append(create(context, remaining));
}
