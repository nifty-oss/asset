/* eslint-disable @typescript-eslint/no-shadow */
import { TypedExtension } from '.';
import { Creator, ExtensionType } from '../generated';

export const creators = (creators: Creator[]): TypedExtension => ({
  type: ExtensionType.Creators,
  creators,
});
