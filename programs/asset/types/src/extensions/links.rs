use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionType};

/// Extension to add external links.
///
/// Links are used to attach external (off-chain) resources to an asset. They are
/// specified as name-uri pair of strings.
///   * `name` - name of the link.
///   * `uri` - URI value.
pub struct Links<'a> {
    pub values: Vec<Link<'a>>,
}

impl<'a> ExtensionData<'a> for Links<'a> {
    const TYPE: ExtensionType = ExtensionType::Links;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let mut cursor = 0;
        let mut values = Vec::new();

        while cursor < bytes.len() {
            let link = Link::from_bytes(&bytes[cursor..]);
            cursor += link.length();
            values.push(link);
        }
        Self { values }
    }

    fn length(&self) -> usize {
        self.values.iter().map(|link| link.length()).sum()
    }
}

impl Debug for Links<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Links")
            .field("values", &self.values)
            .finish()
    }
}

/// Link information.
pub struct Link<'a> {
    /// Name of the link.
    pub name: U8PrefixStr<'a>,

    /// URI value.
    pub uri: U8PrefixStr<'a>,
}

impl<'a> Link<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);
        let uri = U8PrefixStr::from_bytes(&bytes[name.size()..]);
        Self { name, uri }
    }

    pub fn length(&self) -> usize {
        self.name.size() + self.uri.size()
    }
}

impl Debug for Link<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Link")
            .field("name", &self.name.as_str())
            .field("uri", &self.uri.as_str())
            .finish()
    }
}

/// Builder for a `Links` extension.
#[derive(Default)]
pub struct LinksBuilder(Vec<u8>);

impl LinksBuilder {
    /// Add a new attribute to the extension.
    pub fn add(&mut self, name: &str, uri: &str) {
        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; name.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(name);

        // add the length of the value + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; uri.len() + 1]);
        let mut value_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        value_str.copy_from_str(uri);
    }
}

impl ExtensionBuilder for LinksBuilder {
    const TYPE: ExtensionType = ExtensionType::Links;

    fn build(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for LinksBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::{ExtensionData, Links, LinksBuilder};

    #[test]
    fn test_add() {
        let mut builder = LinksBuilder::default();
        builder.add(
            "metadata",
            "https://arweave.net/2eyYRZpFXeXrNyA17Y8QvSfQV9rNkzAqXZa7ko7MBNA",
        );
        builder.add(
            "image",
            "https://arweave.net/aFnc6QVyRR-gVx6pKYSFu0MiwijQzFdU4fMSuApJqms",
        );
        let links = Links::from_bytes(&builder);

        assert_eq!(links.values.len(), 2);
        assert_eq!(links.values[0].name.as_str(), "metadata");
        assert_eq!(
            links.values[0].uri.as_str(),
            "https://arweave.net/2eyYRZpFXeXrNyA17Y8QvSfQV9rNkzAqXZa7ko7MBNA"
        );
        assert_eq!(links.values[1].name.as_str(), "image");
        assert_eq!(
            links.values[1].uri.as_str(),
            "https://arweave.net/aFnc6QVyRR-gVx6pKYSFu0MiwijQzFdU4fMSuApJqms"
        );
    }
}
