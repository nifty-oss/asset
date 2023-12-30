import {
  Context,
  OptionOrNullable,
  TransactionBuilder,
  some
} from '@metaplex-foundation/umi';
import {
  Attributes,
  ExtensionType,
  Image,
  getAttributesSerializer,
  getImageSerializer,
} from './generated';
import {
  InitializeInstructionAccounts,
  initialize as baseInitialize,
} from './generated/instructions/initialize';

type Extension =
  | ({ type: ExtensionType.Attributes } & Attributes)
  | ({ type: ExtensionType.Image } & { length?: number } & Image);

export const attributes = (input: Attributes): Extension => ({
  type: ExtensionType.Attributes,
  ...input,
});

export const image = (
  input: { data: Array<number> } | { length: number }
): Extension => ({
  type: ExtensionType.Image,
  length: 'length' in input ? input.length : 0,
  data: 'data' in input ? input.data : [],
});

export function initialize(
  context: Pick<Context, 'eddsa' | 'payer' | 'programs'>,
  input: InitializeInstructionAccounts & { extension: Extension }
): TransactionBuilder {
  let data: OptionOrNullable<Uint8Array> = null;
  let length = 0;

  switch (input.extension.type) {
    case ExtensionType.Attributes:
      const bytes = getAttributesSerializer().serialize({
        traits: input.extension.traits,
      });
      data = some(bytes);
      length = bytes.length;
      break;
    case ExtensionType.Image:
      if (input.extension.data.length === 0) {
        length = input.extension.length ?? 0;
      } else {
        const bytes = getImageSerializer().serialize({
          data: input.extension.data,
        });
        data = some(bytes);
        length = bytes.length;
      }
      break;
  }

  return baseInitialize(context, {
    ...input,
    extensionType: input.extension.type,
    length,
    data,
  });
}
