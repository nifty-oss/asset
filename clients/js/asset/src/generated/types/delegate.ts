/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, struct } from '@metaplex-foundation/umi/serializers';
import {
  DelegateRoles,
  DelegateRolesArgs,
  NullablePublicKey,
  NullablePublicKeyArgs,
  getDelegateRolesSerializer,
  getNullablePublicKeySerializer,
} from '../../hooked';

export type Delegate = { address: NullablePublicKey; roles: DelegateRoles };

export type DelegateArgs = {
  address: NullablePublicKeyArgs;
  roles: DelegateRolesArgs;
};

export function getDelegateSerializer(): Serializer<DelegateArgs, Delegate> {
  return struct<Delegate>(
    [
      ['address', getNullablePublicKeySerializer()],
      ['roles', getDelegateRolesSerializer()],
    ],
    { description: 'Delegate' }
  ) as Serializer<DelegateArgs, Delegate>;
}
