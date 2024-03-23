import { TypedExtension } from '.';
import {
  Delegate,
  DelegateInputArgs,
  DelegateRole,
  ExtensionType,
} from '../generated';

export type RoleInput = DelegateInputArgs;

export const subscription = (
  address: Delegate['address'],
  roles: Delegate['roles'] | DelegateRole
): TypedExtension => {
  const roleArray = Array.isArray(roles) ? roles : [roles];
  return {
    type: ExtensionType.Subscription,
    delegate: { address, roles: roleArray },
  };
};
