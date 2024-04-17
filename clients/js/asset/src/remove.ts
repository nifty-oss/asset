import {
  Context,
  PublicKey,
  TransactionBuilder,
} from '@metaplex-foundation/umi';
import { ASSET_PROGRAM_ID } from './generated';
import {
  RemoveInstructionAccounts,
  RemoveInstructionArgs,
  remove as baseRemove,
} from './generated/instructions/remove';

export function remove(
  context: Pick<Context, 'identity' | 'programs'>,
  input: RemoveInstructionAccounts &
    RemoveInstructionArgs & { proxy?: PublicKey }
): TransactionBuilder {
  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseRemove(context, input);

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
