import { UmiPlugin } from '@metaplex-foundation/umi';
import { createBridgeProgram } from './generated';

export const niftyBridge = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createBridgeProgram(), false);
  },
});
