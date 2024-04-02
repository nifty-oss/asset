import {
  Context,
  PublicKey,
  TransactionBuilder,
} from '@metaplex-foundation/umi';
import { ASSET_PROGRAM_ID } from './generated';
import {
  VerifyInstructionAccounts,
  verify as baseVerify,
} from './generated/instructions/verify';

export function verify(
  context: Pick<Context, 'programs'>,
  input: VerifyInstructionAccounts & { proxy?: PublicKey }
): TransactionBuilder {
  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseVerify(context, input);

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
