/* eslint-disable @typescript-eslint/no-shadow */
import { Extension } from '.';
import { Creator, ExtensionType } from '../generated';

export const creators = (creators: Creator[]): Extension => ({
  type: ExtensionType.Creators,
  creators,
});
