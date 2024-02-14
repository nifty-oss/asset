use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType};
use crate::validation::Validatable;

/// Extension to add `symbol` and `uri` attributes to an asset.
///
/// This extension is used to add Token Metadata's commonly used `symbol` and `uri`
/// attributes to an asset.
pub struct Metadata<'a> {
    /// Name of the trait.
    pub symbol: U8PrefixStr<'a>,

    /// Value of the trait.
    pub uri: U8PrefixStr<'a>,
}

impl<'a> ExtensionData<'a> for Metadata<'a> {
    const TYPE: ExtensionType = ExtensionType::Metadata;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let symbol = U8PrefixStr::from_bytes(bytes);
        let uri = U8PrefixStr::from_bytes(&bytes[symbol.size()..]);
        Self { symbol, uri }
    }

    fn length(&self) -> usize {
        self.symbol.size() + self.uri.size()
    }
}

impl Debug for Metadata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenMetadata")
            .field("symbol", &self.symbol.as_str())
            .field("uri", &self.uri.as_str())
            .finish()
    }
}

/// Mutable reference to `Metadata` extension.
///
/// This type is used to modify the `Metadata` extension. Note that the `symbol` and `uri` fields
/// are mutable references to the original bytes, but cannot be increased in size.
pub struct MetadataMut<'a> {
    /// Name of the trait.
    pub symbol: U8PrefixStrMut<'a>,

    /// Value of the trait.
    pub uri: U8PrefixStrMut<'a>,
}

impl<'a> ExtensionDataMut<'a> for MetadataMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Metadata;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        // we need to first determine the size of the symbol to be able to split the bytes
        // into mutable symbol and uri refernces
        let symbol = U8PrefixStr::from_bytes(bytes);
        let size = symbol.size();

        let (symbol, uri) = bytes.split_at_mut(size);
        let symbol = U8PrefixStrMut::from_bytes_mut(symbol);
        let uri = U8PrefixStrMut::from_bytes_mut(uri);
        Self { symbol, uri }
    }
}

impl Validatable for Metadata<'_> {}

/// Builder for an `Attributes` extension.
#[derive(Default)]
pub struct MetadataBuilder(Vec<u8>);

impl MetadataBuilder {
    /// Add a new attribute to the extension.
    pub fn set(&mut self, symbol: &str, uri: &str) {
        // setting the data replaces any existing data
        self.0.clear();

        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; symbol.len() + 1]);
        let mut symbol_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        symbol_str.copy_from_str(symbol);

        // add the length of the value + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; uri.len() + 1]);
        let mut uri_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        uri_str.copy_from_str(uri);
    }
}

impl ExtensionBuilder for MetadataBuilder {
    const TYPE: ExtensionType = ExtensionType::Metadata;

    fn build(&mut self) -> Vec<u8> {
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
    fn test_add() {
        let mut builder = MetadataBuilder::default();
        builder.set(
            "SMB",
            "https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc",
        );
        let metadata = Metadata::from_bytes(&builder);

        assert_eq!(metadata.symbol.as_str(), "SMB");
        assert_eq!(
            metadata.uri.as_str(),
            "https://arweave.net/62Z5yOFbIeFqvoOl-aq75EAGSDzS-GxpIKC2ws5LVDc"
        );
    }
}
