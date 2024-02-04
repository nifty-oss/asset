use nifty_asset::{
    extensions::{Blob, Links},
    JsonCreator,
};
use nifty_asset_types::{
    extensions::{Attributes, Extension, ExtensionData, ExtensionType, Metadata},
    ZeroCopy,
};

use super::*;

pub struct DecodeArgs {
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub field: Option<String>,
}

const ASSET_LEN: usize = 168;

pub fn handle_decode(args: DecodeArgs) -> Result<()> {
    let config = CliConfig::new(None, args.rpc_url)?;

    let data = config.client.get_account_data(&args.asset)?;
    let asset = Asset::from_bytes(&data).unwrap();

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
            "holder" => {
                println!("holder: {:?}", asset.holder);
            }
            "group" => {
                println!("group: {:?}", asset.group);
            }
            "authority" => {
                println!("authority: {:?}", asset.authority);
            }
            "delegate" => {
                println!("delegate: {:?}", asset.delegate);
            }
            "name" => {
                println!("name: {:?}", asset.name);
            }
            _ => {
                println!("Unknown field: {}", field);
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
                println!("Blob: {:?}", blob.content_type.as_str());
                // write to a file based on the content type
                let extension = blob.content_type.as_str().split('/').last().unwrap();
                let filename = format!("blob.{}", extension);
                std::fs::write(filename, blob.data).unwrap();

                println!("Blob placeholder");
            }
            ExtensionType::Creators => {
                let creators: Vec<JsonCreator> = extension_data
                    .chunks(40)
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
            ExtensionType::None => {
                println!("None");
            }
        }

        cursor = extension.boundary() as usize;
    }

    Ok(())
}
