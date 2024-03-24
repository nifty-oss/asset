use nifty_asset::instructions::Unlock;

use super::*;

pub struct UnlockArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub signer_keypair_path: Option<PathBuf>,
}

pub fn handle_unlock(args: UnlockArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let payer_sk = Keypair::from_bytes(&config.keypair.to_bytes())?;

    // Use provided signer keypair, or default to the config keypair.
    let signer_sk = if let Some(signer) = args.signer_keypair_path {
        read_keypair_file(signer)
            .map_err(|err| anyhow!("Failed to read signer keypair file: {}", err))?
    } else {
        Keypair::from_bytes(&config.keypair.to_bytes())?
    };

    let signer = signer_sk.pubkey();
    let asset = args.asset;

    let ix = Unlock { asset, signer }.instruction();

    let sig = send_and_confirm_tx(&config.client, &[&payer_sk, &signer_sk], &[ix])?;

    println!("Unlocking asset {asset} in tx: {sig}");

    Ok(())
}
