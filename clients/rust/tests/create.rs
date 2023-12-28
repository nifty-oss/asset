#![cfg(feature = "test-sbf")]

use borsh::BorshDeserialize;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use tpl_asset::{accounts::Asset, instructions::CreateBuilder};

#[tokio::test]
async fn create() {
    let mut context = ProgramTest::new("asset_program", tpl_asset::ID, None)
        .start_with_context()
        .await;

    // Given a new keypair.

    let mold = Keypair::new();
    let asset = Asset::find_pda(&mold.pubkey()).0;

    let ix = CreateBuilder::new()
        .asset(asset)
        .mold(mold.pubkey())
        .authority(context.payer.pubkey())
        .holder(context.payer.pubkey())
        .payer(context.payer.pubkey())
        .name("name".to_string())
        .symbol("symbol".to_string())
        .instruction();

    // When we create a new account.

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &mold],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Then an account was created with the correct data.

    let account = context.banks_client.get_account(asset).await.unwrap();
    assert!(account.is_some());
    let account = account.unwrap();

    let mut account_data = account.data.as_ref();
    let asset = Asset::deserialize(&mut account_data).unwrap();

    assert_eq!(asset.authority, context.payer.pubkey());
    assert_eq!(asset.holder, context.payer.pubkey());
}
