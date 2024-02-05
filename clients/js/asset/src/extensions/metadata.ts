import { TypedExtension } from '.';
import { ExtensionType, Metadata } from '../generated';

export const metadata = (
  symbol: Metadata['symbol'],
  uri: Metadata['uri']
): TypedExtension => ({
  type: ExtensionType.Metadata,
  symbol,
  uri,
});
