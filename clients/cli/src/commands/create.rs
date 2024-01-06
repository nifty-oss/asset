use super::*;

pub struct CreateArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub name: String,
    pub asset_keypair_path: Option<PathBuf>,
    pub immutable: bool,
    pub owner: Option<Pubkey>,
}

pub fn handle_create(args: CreateArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let asset_sk = if let Some(path) = args.asset_keypair_path {
        read_keypair_file(path).expect("failed to read keypair file")
    } else {
        Keypair::new()
    };
    let authority_sk = config.keypair;

    let asset = asset_sk.pubkey();
    let authority = authority_sk.pubkey();
    let holder = args.owner.unwrap_or(authority);

    let ix_args = CreateInstructionArgs {
        name: args.name,
        standard: Standard::NonFungible,
        mutable: !args.immutable,
    };

    let ix = Create {
        asset,
        authority,
        holder,
        payer: Some(authority),
        system_program: Some(system_program::id()),
    }
    .instruction(ix_args);

    let sig = send_and_confirm_tx(&config.client, &[&authority_sk, &asset_sk], &[ix])?;

    println!("Asset {asset} created in tx: {sig}");

    Ok(())
}
