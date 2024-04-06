import { Context, Pda, PublicKey } from '@metaplex-foundation/umi';
import { publicKey as publicKeySerializer } from '@metaplex-foundation/umi/serializers';

export function findProxiedAssetPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    /** The ephemeral stub to derive the address of the asset */
    stub: PublicKey;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'proxy',
    'Proxy11111111111111111111111111111111111111'
  );
  return context.eddsa.findPda(programId, [
    publicKeySerializer().serialize(seeds.stub),
  ]);
}
