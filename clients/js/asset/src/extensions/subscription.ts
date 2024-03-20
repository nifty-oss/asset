import { TypedExtension } from '.';
import { ExtensionType, Subscription } from '../generated';

export const subscription = (
  authority: Subscription['authority']
): TypedExtension => ({
  type: ExtensionType.Subscription,
  authority,
});
