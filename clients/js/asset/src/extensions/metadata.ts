import { TypedExtension } from '.';
import { ExtensionType, Metadata } from '../generated';

export const metadata = (
  input?: Partial<Pick<Metadata, 'symbol' | 'description' | 'uri' | 'imageUri'>>
): TypedExtension => ({
  type: ExtensionType.Metadata,
  symbol: input?.symbol ?? '',
  description: input?.description ?? '',
  uri: input?.uri ?? '',
  imageUri: input?.imageUri ?? '',
});
