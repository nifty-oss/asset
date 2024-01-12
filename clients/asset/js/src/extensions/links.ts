import { Extension } from '.';
import { ExtensionType, Link } from '../generated';

export const links = (values: Link[]): Extension => ({
  type: ExtensionType.Links,
  values,
});
