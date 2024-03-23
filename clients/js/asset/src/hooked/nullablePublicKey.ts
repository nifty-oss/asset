import { PublicKey, defaultPublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  publicKey,
} from '@metaplex-foundation/umi/serializers';

export type NullablePublicKey = PublicKey | null;

export type NullablePublicKeyArgs = NullablePublicKey;

export function getNullablePublicKeySerializer(): Serializer<
  NullablePublicKeyArgs,
  NullablePublicKey
> {
  return mapSerializer(
    publicKey(),
    (nullablePublicKey) => nullablePublicKey ?? defaultPublicKey(),
    (key) => (key === defaultPublicKey() ? null : key)
  );
}
