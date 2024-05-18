use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Empty string used for backwards compatibility with metadata extension.
const ZERO_STR: [u8; 1] = [0u8];

/// Extension to add metadata values to an asset.
///
/// This extension is used to add Token Metadata's commonly used `symbol`, `description`, `uri`
/// and `image_uri` values to an asset.
pub struct Metadata<'a> {
    /// Symbol for the asset.
    pub symbol: U8PrefixStr<'a>,

    /// Description of the asset.
    pub description: U8PrefixStr<'a>,

    /// "Pointer" URI for external metadata.
    pub uri: U8PrefixStr<'a>,

    /// "Pointer" URI for external image.
    pub image_uri: U8PrefixStr<'a>,
}

impl<'a> ExtensionData<'a> for Metadata<'a> {
    const TYPE: ExtensionType = ExtensionType::Metadata;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let symbol = U8PrefixStr::from_bytes(bytes);
        let mut offset = symbol.size();

        let description = U8PrefixStr::from_bytes(&bytes[offset..]);
        offset += description.size();

        let uri = U8PrefixStr::from_bytes(&bytes[offset..]);
        offset += uri.size();

        let image_uri = if offset >= bytes.len() {
            // backwards compatibility for metadata extension: if there are not enough
            // bytes to read the image_uri, we assume it is empty
            U8PrefixStr::from_bytes(&ZERO_STR)
        } else {
            U8PrefixStr::from_bytes(&bytes[offset..])
        };

        Self {
            symbol,
            description,
            uri,
            image_uri,
        }
    }

    fn length(&self) -> usize {
        self.symbol.size() + self.description.size() + self.uri.size() + self.image_uri.size()
    }
}

impl Debug for Metadata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("symbol", &self.symbol.as_str())
            .field("description", &self.description.as_str())
            .field("uri", &self.uri.as_str())
            .field("image_uri", &self.image_uri.as_str())
            .finish()
    }
}

/// Mutable reference to `Metadata` extension.
///
/// This type is used to modify the `Metadata` extension. Note that the `symbol`, `description` and `uri`
/// fields are mutable references to the original bytes, but cannot increase in size.
pub struct MetadataMut<'a> {
    /// Symbol for the asset.
    pub symbol: U8PrefixStrMut<'a>,

    /// Description of the asset.
    pub description: U8PrefixStrMut<'a>,

    /// "Pointer" URI for external metadata.
    pub uri: U8PrefixStrMut<'a>,

    /// "Pointer" URI for external image.
    pub image_uri: U8PrefixStrMut<'a>,
}

impl<'a> ExtensionDataMut<'a> for MetadataMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Metadata;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        // we need to first determine the size of the prefix str to be able to split
        // the bytes into mutable references

        let symbol = U8PrefixStr::from_bytes(bytes);
        let mut offset = symbol.size();

        let (symbol, remaining) = bytes.split_at_mut(offset);
        let symbol = U8PrefixStrMut::from_bytes_mut(symbol);

        let description = U8PrefixStr::from_bytes(remaining);
        offset = description.size();

        let (description, remaining) = remaining.split_at_mut(offset);
        let description = U8PrefixStrMut::from_bytes_mut(description);

        let uri = U8PrefixStr::from_bytes(remaining);
        offset = uri.size();

        let (uri, image_uri) = remaining.split_at_mut(offset);
        let uri = U8PrefixStrMut::from_bytes_mut(uri);

        let image_uri = if image_uri.is_empty() {
            // backwards compatibility for metadata extension: if there are not enough
            // bytes to read the image_uri, we assume it is empty
            U8PrefixStrMut::from_bytes_mut(unsafe {
                (&ZERO_STR as *const [u8] as *mut [u8]).as_mut().unwrap()
            })
        } else {
            U8PrefixStrMut::from_bytes_mut(image_uri)
        };

        Self {
            symbol,
            description,
            uri,
            image_uri,
        }
    }
}

impl Lifecycle for MetadataMut<'_> {}

/// Builder for a `Metadata` extension.
#[derive(Default)]
pub struct MetadataBuilder(Vec<u8>);

impl MetadataBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    /// Set the metadata values.
    pub fn set(
        &mut self,
        symbol: Option<&str>,
        description: Option<&str>,
        uri: Option<&str>,
        image_uri: Option<&str>,
    ) -> &mut Self {
        // setting the data replaces any existing data
        self.0.clear();

        // add the length of the symbol + prefix to the data buffer.
        let cursor = self.0.len();

        if let Some(symbol) = symbol {
            self.0.append(&mut vec![0u8; symbol.len() + 1]);
            let mut symbol_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
            symbol_str.copy_from_str(symbol);
        } else {
            self.0.append(&mut vec![0u8; 1]);
        }

        // add the length of the description + prefix to the data buffer.
        let cursor = self.0.len();

        if let Some(description) = description {
            self.0.append(&mut vec![0u8; description.len() + 1]);
            let mut description_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
            description_str.copy_from_str(description);
        } else {
            self.0.append(&mut vec![0u8; 1]);
        }

        // add the length of the uri + prefix to the data buffer.
        let cursor = self.0.len();

        if let Some(uri) = uri {
            self.0.append(&mut vec![0u8; uri.len() + 1]);
            let mut uri_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
            uri_str.copy_from_str(uri);
        } else {
            self.0.append(&mut vec![0u8; 1]);
        }

        // add the length of the image_uri + prefix to the data buffer.
        let cursor = self.0.len();

        if let Some(image_uri) = image_uri {
            self.0.append(&mut vec![0u8; image_uri.len() + 1]);
            let mut uri_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
            uri_str.copy_from_str(image_uri);
        } else {
            self.0.append(&mut vec![0u8; 1]);
        }

        self
    }
}

impl<'a> ExtensionBuilder<'a, Metadata<'a>> for MetadataBuilder {
    fn build(&'a self) -> Metadata<'a> {
        Metadata::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for MetadataBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::{
        ExtensionBuilder, ExtensionData, ExtensionDataMut, Metadata, MetadataBuilder, MetadataMut,
    };

    #[test]
    fn test_set() {
        let mut builder = MetadataBuilder::default();
        builder.set(
            Some("SMB"),
            None,
            Some("https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc"),
            None,
        );
        let metadata = Metadata::from_bytes(&builder);

        assert_eq!(metadata.symbol.as_str(), "SMB");
        assert_eq!(
            metadata.uri.as_str(),
            "https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc"
        );
    }

    #[test]
    fn test_set_image_uri() {
        let mut builder = MetadataBuilder::default();
        builder.set(
            Some("SMB"),
            None,
            None,
            Some("https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc"),
        );
        let metadata = Metadata::from_bytes(&builder);

        assert_eq!(metadata.symbol.as_str(), "SMB");
        assert_eq!(
            metadata.image_uri.as_str(),
            "https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc"
        );
    }

    #[test]
    fn test_compatibility() {
        let mut builder = MetadataBuilder::default();
        builder.set(Some("SMB"), None, None, None);
        let mut data = builder.data();
        let length = data.len() - 1;

        // remove the last byte to simulate the old metadata extension
        let metadata = Metadata::from_bytes(&data[..length]);

        assert_eq!(metadata.symbol.as_str(), "SMB");
        assert_eq!(metadata.image_uri.as_str(), "");

        let metadata = MetadataMut::from_bytes_mut(&mut data[..length]);

        assert_eq!(metadata.symbol.as_str(), "SMB");
        assert_eq!(metadata.image_uri.as_str(), "");
    }
}
