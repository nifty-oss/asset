use nifty_asset::instructions::Unlock;

use super::*;

pub struct UnlockArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub delegate_keypair_path: Option<PathBuf>,
}

pub fn handle_unlock(args: UnlockArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let payer_sk = Keypair::from_bytes(&config.keypair.to_bytes())?;

    // Use provided delegate keypair, or default to the signer.
    let delegate_sk = if let Some(delegate) = args.delegate_keypair_path {
        read_keypair_file(delegate)
            .map_err(|err| anyhow!("Failed to read delegate keypair file: {}", err))?
    } else {
        Keypair::from_bytes(&config.keypair.to_bytes())?
    };

    let delegate = delegate_sk.pubkey();
    let asset = args.asset;

    let ix = Unlock { asset, delegate }.instruction();

    let sig = send_and_confirm_tx(&config.client, &[&payer_sk, &delegate_sk], &[ix])?;

    println!("Unlocking asset {asset} in tx: {sig}");

    Ok(())
}
