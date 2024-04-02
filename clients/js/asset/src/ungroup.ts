import {
  Context,
  PublicKey,
  TransactionBuilder,
} from '@metaplex-foundation/umi';
import { ASSET_PROGRAM_ID } from './generated';
import {
  UngroupInstructionAccounts,
  ungroup as baseUngroup,
} from './generated/instructions/ungroup';

export function ungroup(
  context: Pick<Context, 'identity' | 'programs'>,
  input: UngroupInstructionAccounts & { proxy?: PublicKey }
): TransactionBuilder {
  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseUngroup(context, input);

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
