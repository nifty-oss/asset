import { Extension } from '.';
import { ExtensionType } from '../generated';

export const blob = (
  contentType: string,
  data: number[] | Uint8Array
): Extension => ({
  type: ExtensionType.Blob,
  contentType,
  data: data instanceof Uint8Array ? Array.from(data) : data,
});
