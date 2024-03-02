import { TypedExtension } from '.';
import { ExtensionType, Grouping } from '../generated';

export const grouping = (
  maxSize: Grouping['size'] | number = 0n,
  currentSize: Grouping['size'] | number = 0n
): TypedExtension => ({
  type: ExtensionType.Grouping,
  size: BigInt(currentSize),
  maxSize: BigInt(maxSize),
});
