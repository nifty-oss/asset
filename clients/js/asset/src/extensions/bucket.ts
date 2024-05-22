import { TypedExtension } from '.';
import { ExtensionType } from '../generated';

export const bucket = (data: number[] | Uint8Array): TypedExtension => ({
  type: ExtensionType.Bucket,
  data: data instanceof Uint8Array ? Array.from(data) : data,
});
