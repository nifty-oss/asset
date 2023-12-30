import {
  Context,
  OptionOrNullable,
  TransactionBuilder,
  isSome,
  some,
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
  | ({ type: ExtensionType.Image } & Image);

export const attributes = (input: Attributes): Extension => ({
  type: ExtensionType.Attributes,
  ...input,
});

export const image = (input: Image): Extension => ({
  type: ExtensionType.Image,
  ...input,
});

export function initialize(
  context: Pick<Context, 'eddsa' | 'payer' | 'programs'>,
  input: InitializeInstructionAccounts & { extension: Extension }
): TransactionBuilder {
  let data: OptionOrNullable<Uint8Array> = null;

  switch (input.extension.type) {
    case ExtensionType.Attributes:
      data = some(
        getAttributesSerializer().serialize({ traits: input.extension.traits })
      );
      break;
    case ExtensionType.Image:
      data = some(
        getImageSerializer().serialize({ data: input.extension.data })
      );
      break;
  }

  const length = isSome(data) ? data.value.length : 0;

  return baseInitialize(context, {
    ...input,
    extensionType: input.extension.type,
    length,
    data,
  });
}
