use nifty_asset::{
    instructions::{Revoke, RevokeInstructionArgs},
    types::{DelegateInput, DelegateRole},
};

use super::*;

pub struct RevokeArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub role: Vec<String>,
    pub all: bool,
}

pub fn handle_revoke(args: RevokeArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let signer_sk = config.keypair;
    let signer = signer_sk.pubkey();
    let asset = args.asset;

    let roles = args
        .role
        .iter()
        .map(|role| match role.to_lowercase().as_str() {
            "burn" => DelegateRole::Burn,
            "lock" => DelegateRole::Lock,
            "transfer" => DelegateRole::Transfer,
            _ => panic!("Invalid role: {}", role),
        })
        .collect();

    let args = RevokeInstructionArgs {
        delegate_input: if args.all {
            DelegateInput::All
        } else {
            DelegateInput::Some { roles }
        },
    };

    let ix = Revoke { asset, signer }.instruction(args);

    let sig = send_and_confirm_tx(&config.client, &[&signer_sk], &[ix])?;

    println!("Revoking the delegate on asset {asset} in tx: {sig}");

    Ok(())
}
