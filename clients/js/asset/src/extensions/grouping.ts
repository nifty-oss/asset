import { TypedExtension } from '.';
import { ExtensionType, Grouping } from '../generated';

export const grouping = (
  maxSize: Grouping['size'] | number = 0n
): TypedExtension => ({
  type: ExtensionType.Grouping,
  size: 0n,
  maxSize: BigInt(maxSize),
});
