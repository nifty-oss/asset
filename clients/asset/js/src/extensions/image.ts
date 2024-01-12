import { Extension } from '.';
import { ExtensionType } from '../generated';

export const image = (data: number[] | Uint8Array): Extension => ({
  type: ExtensionType.Image,
  data: data instanceof Uint8Array ? Array.from(data) : data,
});
