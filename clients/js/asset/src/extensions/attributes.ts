import { TypedExtension } from '.';
import { Attributes, ExtensionType } from '../generated';

export const attributes = (values: Attributes['values']): TypedExtension => ({
  type: ExtensionType.Attributes,
  values,
});
