use std::{fmt, ops::Deref};

use nifty_asset::{
    extensions::{Blob, Links, Manager, Royalties},
    types::Creator,
    JsonCreator,
};
use nifty_asset_types::{
    constraints::{
        And, Constraint, FromBytes, Not, Operator, OperatorType, Or, OwnedBy, PubkeyMatch,
    },
    extensions::{Attributes, Extension, ExtensionData, ExtensionType, Grouping, Metadata},
    podded::ZeroCopy,
};
use serde_json::{json, Value};

use super::*;

pub struct DecodeArgs {
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub field: Option<String>,
    pub raw: bool,
}

const ASSET_LEN: usize = 168;

// Use a wrapper struct to override the debug implementation on Asset
// to print a str instead of bytes for the name field.
pub struct AssetWrapper(pub Asset);

impl fmt::Debug for AssetWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = name_to_string(&self.name);

        f.debug_struct("Asset")
            .field("discriminator", &self.0.discriminator)
            .field("state", &self.0.state)
            .field("standard", &self.0.standard)
            .field("mutable", &self.0.mutable)
            .field("owner", &self.0.owner)
            .field("group", &self.0.group)
            .field("authority", &self.0.authority)
            .field("delegate", &self.0.delegate)
            .field("name", &name)
            .finish()
    }
}

impl AssetWrapper {
    pub fn name(&self) -> String {
        name_to_string(&self.name)
    }
}

impl Deref for AssetWrapper {
    type Target = Asset;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn name_to_string(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes)
        .unwrap_or("[invalid UTF-8]")
        .trim_end_matches('\0')
        .to_string()
}

pub fn handle_decode(args: DecodeArgs) -> Result<()> {
    let config = CliConfig::new(None, args.rpc_url)?;

    let data = config.client.get_account_data(&args.asset)?;

    if args.raw {
        println!("{:?}", data);
        return Ok(());
    }

    let asset = Asset::from_bytes(&data).unwrap();
    let asset = AssetWrapper(asset);

    if let Some(field) = args.field {
        match field.to_lowercase().as_str() {
            "discriminator" => {
                println!("discriminator: {:?}", asset.discriminator);
            }
            "state" => {
                println!("state: {:?}", asset.state);
            }
            "standard" => {
                println!("standard: {:?}", asset.standard);
            }
            "mutable" => {
                println!("mutable: {:?}", asset.mutable);
            }
            "owner" => {
                println!("owner: {:?}", asset.owner);
            }
            "group" => {
                println!("group: {:?}", asset.group);
            }
            "authority" => {
                println!("authority: {:?}", asset.authority);
            }
            "delegate" => {
                println!("delegate address: {:?}", asset.delegate.address);
                println!("delegate roles: {:?}", asset.delegate.roles);
            }
            "name" => {
                println!("name: {:?}", asset.name());
            }
            _ => {
                println!("Unknown field: {:?}", field);
            }
        }
        return Ok(());
    } else {
        println!("Asset: {:#?}", asset);
    }

    let mut cursor = ASSET_LEN;

    // Decode extensions.
    while cursor < data.len() {
        let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);
        let extension_type = extension.extension_type();
        let extension_length = extension.length();

        let start = cursor + Extension::LEN;
        let end = start + extension_length as usize;

        let extension_data = &data[start..end];

        match extension_type {
            ExtensionType::Attributes => {
                let attributes: Attributes = Attributes::from_bytes(extension_data);
                println!("{attributes:#?}");
            }
            ExtensionType::Blob => {
                let blob: Blob = Blob::from_bytes(extension_data);
                // write to a file based on the content type
                let extension = blob.content_type.as_str().split('/').last().unwrap();
                let filename = format!("blob.{}", extension);
                std::fs::write(filename, blob.data).unwrap();

                println!("Blob: {:?}", blob.content_type.as_str());
            }
            ExtensionType::Creators => {
                let creators: Vec<JsonCreator> = extension_data
                    .chunks(std::mem::size_of::<Creator>())
                    .map(JsonCreator::from_data)
                    .collect();
                println!("{creators:#?}");
            }
            ExtensionType::Links => {
                let links: Links = Links::from_bytes(extension_data);
                println!("{links:#?}");
            }
            ExtensionType::Metadata => {
                let metadata: Metadata = Metadata::from_bytes(extension_data);
                println!("{metadata:#?}");
            }
            ExtensionType::Grouping => {
                let grouping: Grouping = Grouping::from_bytes(extension_data);
                println!("{grouping:#?}");
            }
            ExtensionType::Royalties => {
                let royalties: Royalties = Royalties::from_bytes(extension_data);
                let constraint = royalties.constraint;
                let basis_points = royalties.basis_points;

                println!("royalties:");
                println!("basis points:{:#?}", basis_points);

                // Basis Points: u64
                let index = std::mem::size_of::<u64>();

                let constraints = handle_constraints(&constraint, index, extension_data);
                println!("Constraints: {constraints:#?}");
            }
            ExtensionType::Manager => {
                let manager: Manager = Manager::from_bytes(extension_data);
                let delegate = manager.delegate;
                println!("authority: {delegate:#?}");
            }
            ExtensionType::None => {
                println!("None");
            }
        }

        cursor = extension.boundary() as usize;
    }

    Ok(())
}

fn handle_constraints(constraint: &Constraint, mut index: usize, extension_data: &[u8]) -> Value {
    let operator_type = constraint.operator.operator_type();
    let constraint_size = constraint.operator.size() as usize;
    // Operator: [u32; 2]
    index += std::mem::size_of::<Operator>();

    match operator_type {
        OperatorType::And => {
            let and = And::from_bytes(&extension_data[index..]);
            let constraints: Vec<Value> = and
                .constraints
                .iter()
                .map(|constraint| handle_constraints(constraint, index, extension_data))
                .collect();
            json!({
                "AND": constraints
            })
        }
        OperatorType::Not => {
            let constraint = Not::from_bytes(&extension_data[index..]);
            json!({
                "NOT": handle_constraints(&constraint.constraint, index, extension_data)
            })
        }
        OperatorType::Or => {
            let or = Or::from_bytes(&extension_data[index..]);
            let constraints: Vec<Value> = or
                .constraints
                .iter()
                .map(|constraint| handle_constraints(constraint, index, extension_data))
                .collect();
            json!({
                "OR": constraints
            })
        }
        OperatorType::OwnedBy => {
            let owned_by = OwnedBy::from_bytes(&extension_data[index..index + constraint_size]);
            json!({
                "OWNED_BY": {
                    "account": owned_by.account.to_string(),
                    "owners": owned_by.owners.iter().map(|owner| owner.to_string()).collect::<Vec<String>>()
                }
            })
        }
        OperatorType::PubkeyMatch => {
            let pubkey_match =
                PubkeyMatch::from_bytes(&extension_data[index..index + constraint_size]);
            json!({
                "PUBKEY_MATCH": {
                    "account": pubkey_match.account.to_string(),
                    "pubkeys": pubkey_match.pubkeys.iter().map(|pubkey| pubkey.to_string()).collect::<Vec<String>>()
                }
            })
        }
        OperatorType::Empty => {
            json!({ "EMPTY": {} })
        }
    }
}
