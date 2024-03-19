use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to add attributes (traits) to an asset – e.g., `"head": "bald"`.
///
/// A trait is a name-value pair of strings:
///   * `name` - name of the attribute.
///   * `value` - value of the attribute.
pub struct Attributes<'a> {
    traits: Vec<Trait<'a>>,
}

impl Attributes<'_> {
    /// Get the value of a trait by name.
    ///
    /// If no value is found under the `name`, returns `None`.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.traits
            .iter()
            .find(|t| t.name.as_str() == name)
            .map(|t| t.value.as_str())
    }
}

impl<'a> Deref for Attributes<'a> {
    type Target = Vec<Trait<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.traits
    }
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

pub struct AttributesMut<'a> {
    traits: Vec<TraitMut<'a>>,
}

impl<'a> Deref for AttributesMut<'a> {
    type Target = Vec<TraitMut<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.traits
    }
}

impl<'a> ExtensionDataMut<'a> for AttributesMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let mut traits = Vec::new();
        // mutable reference to the current bytes
        let mut bytes = bytes;

        while !bytes.is_empty() {
            let t = Trait::from_bytes(bytes);
            let cursor = t.length();

            let (current, remainder) = bytes.split_at_mut(cursor);
            let t = TraitMut::from_bytes_mut(current);
            bytes = remainder;

            traits.push(t);
        }
        Self { traits }
    }
}

impl Lifecycle for AttributesMut<'_> {}

pub struct TraitMut<'a> {
    /// Name of the trait.
    pub name: U8PrefixStrMut<'a>,

    /// Value of the trait.
    pub value: U8PrefixStrMut<'a>,
}

impl<'a> TraitMut<'a> {
    pub fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);
        let name_size = name.size();

        let (name, value) = bytes.split_at_mut(name_size);

        let name = U8PrefixStrMut::from_bytes_mut(name);
        let value = U8PrefixStrMut::from_bytes_mut(value);
        Self { name, value }
    }

    pub fn length(&self) -> usize {
        self.name.size() + self.value.size()
    }
}

/// Builder for an `Attributes` extension.
#[derive(Default)]
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

impl<'a> ExtensionBuilder<'a, Attributes<'a>> for AttributesBuilder {
    fn build(&self) -> Attributes {
        Attributes::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
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
    use crate::extensions::{AttributesBuilder, ExtensionBuilder};

    #[test]
    fn test_add() {
        let mut builder = AttributesBuilder::default();
        builder.add("head", "bald");
        builder.add("hat", "wizard");
        let attributes = builder.build();

        assert_eq!(attributes.traits.len(), 2);
        assert_eq!(attributes.traits[0].name.as_str(), "head");
        assert_eq!(attributes.traits[0].value.as_str(), "bald");
        assert_eq!(attributes.traits[1].name.as_str(), "hat");
        assert_eq!(attributes.traits[1].value.as_str(), "wizard");
    }
}
