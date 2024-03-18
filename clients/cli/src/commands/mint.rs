use std::fs::File;

use nifty_asset::{AssetArgs, AssetFile, ExtensionArgs, MintAccounts, MintIxArgs};

use super::*;

pub struct MintArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset_file_path: PathBuf,
}

pub async fn handle_mint(args: MintArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let asset_data: AssetFile = serde_json::from_reader(File::open(args.asset_file_path)?)?;

    let asset_sk = if let Some(path) = asset_data.asset_keypair_path {
        read_keypair_file(path).expect("failed to read keypair file")
    } else {
        Keypair::new()
    };
    let authority_sk = config.keypair;

    let asset = asset_sk.pubkey();
    let owner = asset_data.owner;

    let accounts = MintAccounts {
        asset,
        owner,
        payer: Some(authority_sk.pubkey()),
    };
    let asset_args = AssetArgs {
        name: asset_data.name,
        standard: Standard::NonFungible,
        mutable: asset_data.mutable,
    };

    let extension_args = asset_data
        .extensions
        .iter()
        .map(|extension| ExtensionArgs {
            extension_type: extension.extension_type.clone(),
            data: extension.value.clone().into_data(),
            chunked: true,
        })
        .collect::<Vec<ExtensionArgs>>();

    let instructions = mint(MintIxArgs {
        accounts,
        asset_args,
        extension_args,
    })?;

    for instruction in instructions {
        let sig = send_and_confirm_tx(&config.client, &[&authority_sk, &asset_sk], &[instruction])?;
        println!("sig: {}", sig);
    }

    println!("Mint asset: {}", asset);

    Ok(())
}
