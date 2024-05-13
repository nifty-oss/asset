import {
  Context,
  PublicKey,
  TransactionBuilder,
} from '@metaplex-foundation/umi';
import { ASSET_PROGRAM_ID } from './generated';
import {
  resize as baseResize,
  ResizeInstructionAccounts,
  ResizeInstructionArgs,
} from './generated/instructions/resize';

export function resize(
  context: Pick<Context, 'programs' | 'identity' | 'payer'>,
  input: ResizeInstructionAccounts &
    ResizeInstructionArgs & { proxy?: PublicKey }
): TransactionBuilder {
  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseResize(context, input);

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
