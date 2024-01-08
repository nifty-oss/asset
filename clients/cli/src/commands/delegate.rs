use nifty_asset::{
    instructions::{Delegate, DelegateInstructionArgs},
    types::DelegateRole,
};

use super::*;

pub struct DelegateArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub delegate: Pubkey,
    pub role: Vec<String>,
}

pub fn handle_delegate(args: DelegateArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let holder_sk = config.keypair;

    let holder = holder_sk.pubkey();
    let asset = args.asset;
    let delegate = args.delegate;

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

    let args = DelegateInstructionArgs { args: roles };

    let ix = Delegate {
        asset,
        holder,
        delegate,
    }
    .instruction(args);

    let sig = send_and_confirm_tx(&config.client, &[&holder_sk], &[ix])?;

    println!("Setting {delegate} as a delegate on asset {asset} in tx: {sig}");

    Ok(())
}
