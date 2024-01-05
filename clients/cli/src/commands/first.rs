use super::*;

pub struct FirstArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub value: String,
}

pub fn handle_first(args: FirstArgs) -> Result<()> {
    let _config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    Ok(())
}
