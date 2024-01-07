use super::*;

pub struct DecodeArgs {
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub field: Option<String>,
}

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
    } else {
        println!("Asset: {:#?}", asset);
    }

    Ok(())
}
