import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilderGroup,
  transactionBuilder,
  transactionBuilderGroup,
} from '@metaplex-foundation/umi';
import { write as baseWrite } from './generated/instructions/write';

const DEFAULT_CHUNK_SIZE = 850;

export type WriteInstruction = {
  /** Address to derive the PDA from */
  asset: Signer;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
  /** Data to write to the buffer. */
  data: Uint8Array;
  /**
   * The size of each chunk to write to the buffer.
   * @default `850`
   */
  chunkSize?: number;
};

export const write = (
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: WriteInstruction
): TransactionBuilderGroup => {
  const payer = input.payer ?? context.payer;
  const chunkSize = input.chunkSize ?? DEFAULT_CHUNK_SIZE;

  const bufferSize = input.data.length;
  const numberOfWrites = Math.ceil(bufferSize / chunkSize);
  const writeInstructions = Array.from(
    { length: numberOfWrites },
    (_, index) => {
      const slice = input.data.slice(
        index * chunkSize,
        Math.min((index + 1) * chunkSize, input.data.length)
      );
      return baseWrite(context, {
        asset: input.asset,
        payer,
        bytes: slice,
        overwrite: index === 0,
      });
    }
  );

  return transactionBuilderGroup(
    transactionBuilder()
      .add(writeInstructions)
      .unsafeSplitByTransactionSize(context)
  ).sequential();
};
