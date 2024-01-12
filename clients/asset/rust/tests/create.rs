#![cfg(feature = "test-sbf")]

use borsh::BorshDeserialize;
use nifty_asset::{
    accounts::Asset,
    instructions::CreateBuilder,
    types::{Discriminator, Standard, State},
};
use solana_program::system_program;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod create {

    use super::*;

    #[tokio::test]
    async fn create() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let ix = CreateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey())
            .holder(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .name("name".to_string())
            .instruction();

        // When we create a new asset.

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &asset],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

        // Then an asset was created with the correct data.

        let account = context
            .banks_client
            .get_account(asset.pubkey())
            .await
            .unwrap();
        assert!(account.is_some());
        let account = account.unwrap();

        let mut account_data = account.data.as_ref();
        let asset = Asset::deserialize(&mut account_data).unwrap();

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, Standard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.holder, context.payer.pubkey());
    }
}
