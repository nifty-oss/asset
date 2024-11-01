#[cfg(feature = "parse")]
use {
    flate2::bufread::ZlibDecoder,
    goblin::elf::Elf,
    serde_json::Value,
    std::io::Read,
    std::str::{from_utf8, FromStr},
};

/// Name of the section containing the IDL type value.
pub const IDL_TYPE_SECTION: &str = ".idl.type";

/// Name of the section containing the IDL data.
pub const IDL_DATA_SECTION: &str = ".idl.data";

/// `str` representation of the Anchor IDL type.
const ANCHOR_IDL_TYPE: &str = "anchor";

/// `str` representation of the Codama IDL type.
const CODAMA_IDL_TYPE: &str = "codama";

/// Defines the IDL type.
#[derive(Clone, Debug)]
pub enum IdlType {
    Anchor,
    Codama,
}

impl IdlType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            IdlType::Anchor => ANCHOR_IDL_TYPE,
            IdlType::Codama => CODAMA_IDL_TYPE,
        }
    }
}

impl std::fmt::Display for IdlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IdlType::Anchor => write!(f, "{ANCHOR_IDL_TYPE}"),
            IdlType::Codama => write!(f, "{CODAMA_IDL_TYPE}"),
        }
    }
}

impl std::str::FromStr for IdlType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            ANCHOR_IDL_TYPE => Ok(IdlType::Anchor),
            CODAMA_IDL_TYPE => Ok(IdlType::Codama),
            _ => Err("Invalid IDL type"),
        }
    }
}

/// Parses the IDL data from the program binary.
#[cfg(feature = "parse")]
pub fn parse_idl_from_program_binary(buffer: &[u8]) -> goblin::error::Result<(IdlType, Value)> {
    let elf = Elf::parse(buffer)?;

    let mut idl_type: Option<IdlType> = None;
    let mut idl_json: Option<Value> = None;

    // Iterate over section headers and retrieve the IDL data.
    for sh in &elf.section_headers {
        let name = elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("<invalid>");

        match name {
            IDL_DATA_SECTION => {
                let (location, size) = get_section_data_offset(buffer, sh.sh_offset as usize);

                let slice = &buffer[location..location + size];
                let mut compressed_data = ZlibDecoder::new(slice);
                let mut data = Vec::new();
                compressed_data.read_to_end(&mut data).unwrap();

                idl_json = Some(serde_json::from_slice(&data).unwrap());
            }
            IDL_TYPE_SECTION => {
                let (location, size) = get_section_data_offset(buffer, sh.sh_offset as usize);
                let slice = &buffer[location..location + size];

                idl_type = Some(IdlType::from_str(from_utf8(slice).unwrap()).unwrap());
            }
            // Ignore other sections.
            _ => (),
        }
    }

    if idl_type.is_some() && idl_json.is_some() {
        #[allow(clippy::unnecessary_unwrap)]
        Ok((idl_type.unwrap(), idl_json.unwrap()))
    } else {
        // Returns an error if we could not find the IDL information.
        Err(goblin::error::Error::Malformed(
            "Could not find .idl.* sections".to_string(),
        ))
    }
}

/// Retrieves the location and size of an ELF section data.
#[cfg(feature = "parse")]
#[inline(always)]
fn get_section_data_offset(buffer: &[u8], offset: usize) -> (usize, usize) {
    let slice = &buffer[offset + 4..offset + 8];
    let location = u32::from_le_bytes(slice.try_into().unwrap());

    let slice = &buffer[offset + 8..offset + 16];
    let size = u64::from_le_bytes(slice.try_into().unwrap());

    (location as usize, size as usize)
}
