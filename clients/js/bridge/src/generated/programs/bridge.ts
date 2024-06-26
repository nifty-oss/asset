/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
} from '@metaplex-foundation/umi';
import { getBridgeErrorFromCode, getBridgeErrorFromName } from '../errors';

export const BRIDGE_PROGRAM_ID =
  'BridgezKrNugsZwTcyAMYba643Z93RzC2yN1Y24LwAkm' as PublicKey<'BridgezKrNugsZwTcyAMYba643Z93RzC2yN1Y24LwAkm'>;

export function createBridgeProgram(): Program {
  return {
    name: 'bridge',
    publicKey: BRIDGE_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getBridgeErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getBridgeErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getBridgeProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('bridge', clusterFilter);
}

export function getBridgeProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'bridge',
    BRIDGE_PROGRAM_ID,
    clusterFilter
  );
}
