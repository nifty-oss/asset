import { TypedExtension } from '.';
import { Delegate, DelegateRole, ExtensionType } from '../generated';

export const manager = (
  address: Delegate['address'],
  roles: Delegate['roles'] | DelegateRole
): TypedExtension => {
  const roleArray = Array.isArray(roles) ? roles : [roles];
  return {
    type: ExtensionType.Manager,
    delegate: { address, roles: roleArray },
  };
};
