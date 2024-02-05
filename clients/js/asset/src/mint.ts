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
} from './generated';
import { initialize } from './initialize';

export function mint(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: CreateInstructionAccounts &
    CreateInstructionArgs & { extensions?: TypedExtension[] }
): TransactionBuilderGroup {
  let builder = transactionBuilderGroup();

  if (input.extensions) {
    input.extensions.forEach((extension) => {
      builder = builder.append(
        initialize(context, {
          ...input,
          extension,
        }).builders
      );
    });
  }

  return builder.append(create(context, input));
}
