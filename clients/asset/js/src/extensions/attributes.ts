import { Extension } from '.';
import { Attributes, ExtensionType } from '../generated';

export const attributes = (traits: Attributes['traits']): Extension => ({
  type: ExtensionType.Attributes,
  traits,
});
