use nifty_asset::instructions::Transfer;

use super::*;

pub struct TransferArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub recipient: Pubkey,
}

pub fn handle_transfer(args: TransferArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let signer_sk = config.keypair;

    let signer = signer_sk.pubkey();
    let asset = args.asset;
    let recipient = args.recipient;

    let ix = Transfer {
        asset,
        signer,
        recipient,
    }
    .instruction();

    let sig = send_and_confirm_tx(&config.client, &[&signer_sk], &[ix])?;

    println!("Transferring asset {asset} to {recipient} in tx: {sig}");

    Ok(())
}
