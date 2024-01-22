import { TypedExtension } from '.';
import { ExtensionType } from '../generated';

export const blob = (
  contentType: string,
  data: number[] | Uint8Array
): TypedExtension => ({
  type: ExtensionType.Blob,
  contentType,
  data: data instanceof Uint8Array ? Array.from(data) : data,
});
