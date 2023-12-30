import { UmiPlugin } from '@metaplex-foundation/umi';
import { createAssetProgram } from './generated';

export const niftyAsset = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createAssetProgram(), false);
  },
});
