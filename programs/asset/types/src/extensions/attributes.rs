use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionType};

/// Extension to add attributes (traits) to an asset – e.g., `"head": "bald"`.
///
/// A trait is a name-value pair of strings:
///   * `name` - name of the attribute.
///   * `value` - value of the attribute.
pub struct Attributes<'a> {
    pub traits: Vec<Trait<'a>>,
}

impl<'a> ExtensionData<'a> for Attributes<'a> {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let mut cursor = 0;
        let mut traits = Vec::new();

        while cursor < bytes.len() {
            let t = Trait::from_bytes(&bytes[cursor..]);
            cursor += t.length();
            traits.push(t);
        }
        Self { traits }
    }

    fn length(&self) -> usize {
        self.traits.iter().map(|t| t.length()).sum()
    }
}

impl Debug for Attributes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attributes")
            .field("traits", &self.traits)
            .finish()
    }
}

/// Trait information.
pub struct Trait<'a> {
    /// Name of the trait.
    pub name: U8PrefixStr<'a>,

    /// Value of the trait.
    pub value: U8PrefixStr<'a>,
}

impl<'a> Trait<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);
        let value = U8PrefixStr::from_bytes(&bytes[name.size()..]);
        Self { name, value }
    }

    pub fn length(&self) -> usize {
        self.name.size() + self.value.size()
    }
}

impl Debug for Trait<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trait")
            .field("name", &self.name.as_str())
            .field("value", &self.value.as_str())
            .finish()
    }
}

/// Builder for an `Attributes` extension.
pub struct AttributesBuilder(Vec<u8>);

impl AttributesBuilder {
    /// Add a new attribute to the extension.
    pub fn add(&mut self, name: &str, value: &str) {
        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; name.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(name);

        // add the length of the value + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; value.len() + 1]);
        let mut value_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        value_str.copy_from_str(value);
    }
}

impl ExtensionBuilder for AttributesBuilder {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn build(&mut self) -> Vec<u8> {
        std::mem::replace(&mut self.0, Vec::new())
    }
}

impl Default for AttributesBuilder {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Deref for AttributesBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::{Attributes, AttributesBuilder, ExtensionData};

    #[test]
    fn test_add() {
        let mut builder = AttributesBuilder::default();
        builder.add("head", "bald");
        builder.add("hat", "wizard");
        let attributes = Attributes::from_bytes(&builder);

        assert_eq!(attributes.traits.len(), 2);
        assert_eq!(attributes.traits[0].name.as_str(), "head");
        assert_eq!(attributes.traits[0].value.as_str(), "bald");
        assert_eq!(attributes.traits[1].name.as_str(), "hat");
        assert_eq!(attributes.traits[1].value.as_str(), "wizard");
    }
}
