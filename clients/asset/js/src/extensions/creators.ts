/* eslint-disable @typescript-eslint/no-shadow */
import { TypedExtension } from '.';
import { Creator, ExtensionType } from '../generated';

export const creators = (
  creators: Omit<Creator, 'verified' | 'padding'>[]
): TypedExtension => ({
  type: ExtensionType.Creators,
  creators: creators.map((creator) => ({
    ...creator,
    verified: false,
    padding: new Uint8Array(),
  })),
});
