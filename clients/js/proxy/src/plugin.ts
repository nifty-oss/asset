import { UmiPlugin } from '@metaplex-foundation/umi';
import { createProxyProgram } from './generated';

export const niftyProxy = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createProxyProgram(), false);
  },
});
