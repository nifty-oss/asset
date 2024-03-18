use std::{sync::Arc, time::Duration};

use super::*;

use indicatif::ProgressBar;
use tokio::sync::Mutex;

pub struct MintBatchArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset_files_dir: PathBuf,
}

pub struct AssetStruct {
    pub asset: AssetFile,
    pub asset_sk: Keypair,
    pub owner: Pubkey,
}

pub struct AssetResult {
    pub asset_pubkey: Pubkey,
    pub asset_name: String,
    pub tx_result: TxResult,
}

pub enum TxResult {
    Success,
    Failure(String),
}

impl TxResult {
    pub fn is_success(&self) -> bool {
        matches!(self, TxResult::Success)
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, TxResult::Failure(_))
    }

    pub fn get_error(&self) -> Option<&str> {
        match self {
            TxResult::Success => None,
            TxResult::Failure(err) => Some(err),
        }
    }
}

pub async fn handle_mint_batch(args: MintBatchArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    // Get all JSON asset files in the directory
    let asset_files = std::fs::read_dir(args.asset_files_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    let authority_sk = config.keypair;

    let mut instructions = vec![];

    let asset_keys = Arc::new(Mutex::new(vec![]));
    let asset_results = Arc::new(Mutex::new(vec![]));

    for asset_file_path in asset_files {
        let asset_data: AssetFile = serde_json::from_reader(File::open(asset_file_path)?)?;

        let asset_sk = if let Some(path) = &asset_data.asset_keypair_path {
            read_keypair_file(path).expect("failed to read keypair file")
        } else {
            Keypair::new()
        };

        let asset = asset_sk.pubkey();

        asset_keys.lock().await.push(asset_sk);

        asset_results.lock().await.push(AssetResult {
            asset_pubkey: asset,
            asset_name: asset_data.name.clone(),
            tx_result: TxResult::Success,
        });

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

        instructions.push(mint(MintIxArgs {
            accounts,
            asset_args,
            extension_args,
        })?);
    }

    // Instructions for each asset must be submitted serially, but we can parallelize the minting of
    // the assets themselves.

    let mut futures = vec![];

    let client = Arc::new(config.client);
    let authority_sk = Arc::new(authority_sk);

    let pb = ProgressBar::new(instructions.iter().flatten().count() as u64);
    pb.set_message("Minting assets");

    for (i, asset_instructions) in instructions.into_iter().enumerate() {
        let client = client.clone();
        let authority_sk = authority_sk.clone();
        let asset_keys = asset_keys.clone();
        let asset_results = asset_results.clone();

        let pb = pb.clone();

        futures.push(tokio::spawn(async move {
            for instruction in asset_instructions {
                let asset_sk = &asset_keys.lock().await[i];
                let res = send_and_confirm_tx(&client, &[&authority_sk, &asset_sk], &[instruction]);
                pb.inc(1);

                match res {
                    Ok(_) => continue,
                    Err(err) => {
                        let mut results = asset_results.lock().await;
                        results[i].tx_result = TxResult::Failure(err.to_string());
                        break;
                    }
                }
            }
        }));

        // Sleep for a short time to avoid sending transactions too quickly
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    pb.finish_and_clear();

    let pb = ProgressBar::new(asset_results.lock().await.len() as u64);
    pb.set_message("Waiting for confirmations");
    for future in futures {
        future.await?;
        pb.inc(1);
    }
    pb.finish_and_clear();

    let results = asset_results.lock().await;
    for result in results.iter() {
        if result.tx_result.is_failure() {
            println!(
                "Failed to mint asset {}: {}",
                result.asset_name,
                result.tx_result.get_error().unwrap()
            );
        } else {
            println!("Minted asset: {}", result.asset_pubkey);
        }
    }

    Ok(())
}
