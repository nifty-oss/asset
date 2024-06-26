import { PublicKey, defaultPublicKey } from '@metaplex-foundation/umi';
import { TypedExtension } from '.';
import { ExtensionType, Grouping } from '../generated';

export const grouping = (
  maxSize: Grouping['size'] | number = 0n,
  delegate: Grouping['delegate'] | PublicKey = defaultPublicKey()
): TypedExtension => ({
  type: ExtensionType.Grouping,
  size: BigInt(0),
  maxSize: BigInt(maxSize),
  delegate,
});
