use nifty_asset::instructions::Unlock;

use super::*;

pub struct UnlockArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub authority_keypair_path: Option<PathBuf>,
}

pub fn handle_unlock(args: UnlockArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let payer_sk = Keypair::from_bytes(&config.keypair.to_bytes())?;

    // Use provided authority keypair, or default to the signer.
    let authority_sk = if let Some(authority) = args.authority_keypair_path {
        read_keypair_file(authority)
            .map_err(|err| anyhow!("Failed to read authority keypair file: {}", err))?
    } else {
        Keypair::from_bytes(&config.keypair.to_bytes())?
    };

    let authority = authority_sk.pubkey();
    let asset = args.asset;

    let ix = Unlock { asset, authority }.instruction();

    let sig = send_and_confirm_tx(&config.client, &[&payer_sk, &authority_sk], &[ix])?;

    println!("Unlocking asset {asset} in tx: {sig}");

    Ok(())
}
