import {
  Context,
  PublicKey,
  TransactionBuilder,
} from '@metaplex-foundation/umi';
import { ASSET_PROGRAM_ID } from './generated';
import {
  UnverifyInstructionAccounts,
  unverify as baseUnverify,
} from './generated/instructions/unverify';

export function unverify(
  context: Pick<Context, 'programs'>,
  input: UnverifyInstructionAccounts & { proxy?: PublicKey }
): TransactionBuilder {
  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseUnverify(context, input);

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
