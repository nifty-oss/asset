use super::*;

pub struct DecodeArgs {
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
}

pub fn handle_decode(args: DecodeArgs) -> Result<()> {
    let config = CliConfig::new(None, args.rpc_url)?;

    let data = config.client.get_account_data(&args.asset)?;
    let asset = Asset::from_bytes(&data).unwrap();

    println!("Asset: {:#?}", asset);

    Ok(())
}
