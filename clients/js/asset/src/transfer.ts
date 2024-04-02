import {
  Context,
  PublicKey,
  TransactionBuilder,
} from '@metaplex-foundation/umi';
import {
  transfer as baseTransfer,
  TransferInstructionAccounts,
} from './generated/instructions/transfer';
import { ASSET_PROGRAM_ID } from './generated';

export function transfer(
  context: Pick<Context, 'programs'>,
  input: TransferInstructionAccounts & { proxy?: PublicKey }
): TransactionBuilder {
  if (input.proxy) {
    const proxied = context.programs.clone();
    proxied.bind('asset', input.proxy);
    context = { ...context, programs: proxied };
  }

  let ix = baseTransfer(context, input);

  if (input.proxy) {
    ix = ix.addRemainingAccounts({
      pubkey: ASSET_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    });
  }

  return ix;
}
