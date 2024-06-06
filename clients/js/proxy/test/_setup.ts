/* eslint-disable import/no-extraneous-dependencies */
import { createUmi as basecreateUmi } from '@metaplex-foundation/umi-bundle-tests';
import { niftyProxy } from '../src';

export const createUmi = async () => (await basecreateUmi()).use(niftyProxy());

export const STUB_KEY = Uint8Array.from([
  229, 92, 57, 211, 157, 68, 169, 122, 106, 251, 195, 223, 203, 125, 80, 52, 9,
  161, 139, 39, 198, 140, 124, 83, 97, 66, 49, 121, 221, 229, 160, 192, 215, 64,
  198, 193, 21, 158, 198, 203, 241, 229, 179, 162, 229, 129, 109, 151, 51, 135,
  240, 128, 114, 242, 103, 170, 154, 47, 218, 130, 218, 139, 45, 47,
]);

export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
