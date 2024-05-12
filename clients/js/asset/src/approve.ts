import {
  Context,
  PublicKey,
  TransactionBuilder,
} from '@metaplex-foundation/umi';
import { ASSET_PROGRAM_ID } from './generated';
import {
  ApproveInstructionAccounts,
  ApproveInstructionArgs,
  approve as baseApprove,
} from './generated/instructions/approve';

export function approve(
  context: Pick<Context, 'identity' | 'programs'>,
  input: ApproveInstructionAccounts &
    ApproveInstructionArgs & { proxy?: PublicKey }
): TransactionBuilder {
  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseApprove(context, input);

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
