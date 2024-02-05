import { Context, Pda, PublicKey } from '@metaplex-foundation/umi';
import {
  string,
  publicKey as publicKeySerializer,
} from '@metaplex-foundation/umi/serializers';

export function findBridgeAssetPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    /** The address of the mint */
    mint: PublicKey;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'bridge',
    'BridgezKrNugsZwTcyAMYba643Z93RzC2yN1Y24LwAkm'
  );
  return context.eddsa.findPda(programId, [
    string({ size: 'variable' }).serialize('nifty::bridge::asset'),
    publicKeySerializer().serialize(seeds.mint),
  ]);
}
