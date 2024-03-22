use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to add `symbol`, `description` and `uri` values to an asset.
///
/// This extension is used to add Token Metadata's commonly used `symbol`, `description` and `uri`
/// values to an asset.
pub struct Metadata<'a> {
    /// Symbol for the asset.
    pub symbol: U8PrefixStr<'a>,
    /// Description of the asset.
    pub description: U8PrefixStr<'a>,
    /// "Pointer" URI for external metadata.
    pub uri: U8PrefixStr<'a>,
}

impl<'a> ExtensionData<'a> for Metadata<'a> {
    const TYPE: ExtensionType = ExtensionType::Metadata;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let symbol = U8PrefixStr::from_bytes(bytes);
        let description = U8PrefixStr::from_bytes(&bytes[symbol.size()..]);
        let uri = U8PrefixStr::from_bytes(&bytes[symbol.size() + description.size()..]);

        Self {
            symbol,
            description,
            uri,
        }
    }

    fn length(&self) -> usize {
        self.symbol.size() + self.description.size() + self.uri.size()
    }
}

impl Debug for Metadata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("symbol", &self.symbol.as_str())
            .field("description", &self.description.as_str())
            .field("uri", &self.uri.as_str())
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
}

impl<'a> ExtensionDataMut<'a> for MetadataMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Metadata;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        // we need to first determine the size of the prefix str to be able to split
        // the bytes into mutable references

        let symbol = U8PrefixStr::from_bytes(bytes);
        let symbol_size = symbol.size();

        let description = U8PrefixStr::from_bytes(&bytes[symbol_size..]);
        let description_size = description.size();

        let (symbol, remaining) = bytes.split_at_mut(symbol_size);
        let symbol = U8PrefixStrMut::from_bytes_mut(symbol);

        let (description, uri) = remaining.split_at_mut(description_size);
        let description = U8PrefixStrMut::from_bytes_mut(description);
        let uri = U8PrefixStrMut::from_bytes_mut(uri);

        Self {
            symbol,
            description,
            uri,
        }
    }
}

impl Lifecycle for MetadataMut<'_> {}

/// Builder for an `Attributes` extension.
#[derive(Default)]
pub struct MetadataBuilder(Vec<u8>);

impl MetadataBuilder {
    /// Add a new attribute to the extension.
    pub fn set(&mut self, symbol: Option<&str>, description: Option<&str>, uri: Option<&str>) {
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
    use crate::extensions::{ExtensionData, Metadata, MetadataBuilder};

    #[test]
    fn test_set() {
        let mut builder = MetadataBuilder::default();
        builder.set(
            Some("SMB"),
            None,
            Some("https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc"),
        );
        let metadata = Metadata::from_bytes(&builder);

        assert_eq!(metadata.symbol.as_str(), "SMB");
        assert_eq!(
            metadata.uri.as_str(),
            "https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc"
        );
    }
}
