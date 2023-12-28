import { UmiPlugin } from '@metaplex-foundation/umi';
import { createAssetProgram } from './generated';

export const tplAsset = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createAssetProgram(), false);
  },
});