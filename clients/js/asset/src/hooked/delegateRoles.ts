import {
  Serializer,
  mapSerializer,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { DelegateRole, DelegateRoleArgs } from '../generated';

export type DelegateRoles = DelegateRole[];

export type DelegateRolesArgs = DelegateRoleArgs[];

/* eslint no-bitwise: "off" */
export function getDelegateRolesSerializer(): Serializer<
  DelegateRolesArgs,
  DelegateRoles
> {
  return mapSerializer(
    u8(),
    (delegateRoles) =>
      delegateRoles.map((role) => 1 << (role - 1)).reduce((a, b) => a | b, 0),
    (rolesMask) => {
      if (rolesMask === 0) {
        return [DelegateRole.None];
      }
      return Object.keys(DelegateRole)
        .filter((key): key is keyof typeof DelegateRole =>
          Number.isNaN(Number(key))
        )
        .map((key) => DelegateRole[key])
        .filter((value) => (rolesMask & (1 << (value - 1))) !== 0);
    }
  );
}
