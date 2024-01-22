import { TypedExtension } from '.';
import { Attributes, ExtensionType } from '../generated';

export const attributes = (traits: Attributes['traits']): TypedExtension => ({
  type: ExtensionType.Attributes,
  traits,
});
