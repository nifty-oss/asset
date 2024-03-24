#![cfg(feature = "test-sbf")]
pub mod utils;

use nifty_asset::{
    instructions::{CreateBuilder, HandoverBuilder},
    state::{Asset, Discriminator, State},
    ZeroCopy,
};
use nifty_asset_types::state::Standard;
use solana_program_test::{tokio, BanksClientError, ProgramTest};
use solana_sdk::{
    instruction::InstructionError,
    signature::{Keypair, Signer},
    system_program,
    transaction::{Transaction, TransactionError},
};

mod handover {
    use super::*;

    #[tokio::test]
    async fn handover_to_nonsigner_fails() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        let asset_signer = Keypair::new();
        let asset = asset_signer.pubkey();
        let authority_signer = Keypair::new();
        let authority = authority_signer.pubkey();
        let new_authority = Keypair::new().pubkey();

        // Create an asset.
        let ix = CreateBuilder::new()
            .asset(asset)
            .authority(authority, false)
            .owner(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .name("name".to_string())
            .instruction();

        // When we create a new asset.

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &asset_signer],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

        // Then an asset was created with the correct data.

        let account = context.banks_client.get_account(asset).await.unwrap();
        assert!(account.is_some());
        let account = account.unwrap();

        let account_data = account.data.as_ref();
        let asset_account = Asset::load(account_data);

        assert_eq!(asset_account.discriminator, Discriminator::Asset);
        assert_eq!(asset_account.state, State::Unlocked);
        assert_eq!(asset_account.standard, Standard::NonFungible);
        assert_eq!(asset_account.authority, authority);
        assert_eq!(asset_account.owner, context.payer.pubkey());
        // we are not expecting any extension on the account
        assert!(Asset::get_extensions(account_data).is_empty());

        // Handover the asset to a new authority, that is not a signer.
        // This should fail.
        let mut ix = HandoverBuilder::new()
            .asset(asset)
            .authority(authority)
            .new_authority(new_authority)
            .instruction();

        // Manually change the account meta for the new authority to not be a signer.
        ix.accounts[2].is_signer = false;

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &authority_signer], // New authority not a signer
            context.last_blockhash,
        );

        let err = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();

        assert_instruction_error!(err, InstructionError::MissingRequiredSignature);
    }
}
