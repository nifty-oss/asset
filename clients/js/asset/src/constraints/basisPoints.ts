import { Serializer, u64 } from '@metaplex-foundation/umi/serializers';

export type BasisPoints = {
  basisPoints: bigint | number;
};

export function getBasisPointsSerializer(): Serializer<BasisPoints> {
  return {
    description: 'BasisPoints',
    fixedSize: 8,
    maxSize: 8,
    serialize: (value: BasisPoints) => u64().serialize(value.basisPoints),
    deserialize: (buffer: Uint8Array) => {
      const bytes = buffer.subarray(0, 8);
      const bigIntValue = new BigUint64Array(bytes.buffer)[0];
      return [{ basisPoints: bigIntValue }, 8];
    },
  };
}
