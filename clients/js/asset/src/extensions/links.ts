import { TypedExtension } from '.';
import { ExtensionType, Link } from '../generated';

export const links = (values: Link[]): TypedExtension => ({
  type: ExtensionType.Links,
  values,
});
