#![cfg(feature = "test-sbf")]
pub mod utils;

use nifty_asset::{
    errors::AssetError,
    extensions::{Attributes, AttributesBuilder, ExtensionBuilder},
    instructions::{CreateBuilder, UpdateBuilder},
    state::{Asset, Discriminator, Standard, State},
    types::{ExtensionInput, ExtensionType},
    ZeroCopy,
};
use solana_program::system_program;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod update {

    use super::*;

    #[tokio::test]
    async fn update() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let create_ix = CreateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey(), false)
            .owner(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .name("name".to_string())
            .instruction();

        // And we create a new asset.

        let tx = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &asset],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

        let account = context
            .banks_client
            .get_account(asset.pubkey())
            .await
            .unwrap();

        assert!(account.is_some());
        let account = account.unwrap();

        let account_data = account.data.as_ref();
        assert!(Asset::get_extensions(account_data).is_empty());

        let mut attributes = AttributesBuilder::default();
        attributes.add("hat", "nifty");
        let data = attributes.data();

        let update_ix = UpdateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .extension(ExtensionInput {
                extension_type: ExtensionType::Attributes,
                length: data.len() as u32,
                data: Some(data),
            })
            .instruction();

        // When we update the asset.
        let tx = Transaction::new_signed_with_payer(
            &[update_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

        // Then an asset was updated with the correct data.

        let account = context
            .banks_client
            .get_account(asset.pubkey())
            .await
            .unwrap();

        assert!(account.is_some());
        let account = account.unwrap();

        let account_data = account.data.as_ref();
        let asset = Asset::load(account_data);

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, Standard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.owner, context.payer.pubkey());

        // And we are expecting an extension on the account.

        assert!(Asset::get_extensions(account_data).len() == 1);
        let attributes = Asset::get::<Attributes>(account_data).unwrap();

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name.as_str(), "hat");
        assert_eq!(attributes[0].value.as_str(), "nifty");
    }

    #[tokio::test]
    async fn update_with_invalid_extension_length_fails() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let create_ix = CreateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey(), false)
            .owner(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .name("name".to_string())
            .instruction();

        // And we create a new asset.

        let tx = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &asset],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

        let account = context
            .banks_client
            .get_account(asset.pubkey())
            .await
            .unwrap();

        assert!(account.is_some());
        let account = account.unwrap();

        let account_data = account.data.as_ref();
        assert!(Asset::get_extensions(account_data).is_empty());

        let mut attributes = AttributesBuilder::default();
        attributes.add("hat", "nifty");
        let data = attributes.data();

        let update_ix = UpdateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .extension(ExtensionInput {
                extension_type: ExtensionType::Attributes,
                // invalida extension length
                length: 1,
                data: Some(data),
            })
            .instruction();

        // When we update the asset with an invalid extension length.
        let tx = Transaction::new_signed_with_payer(
            &[update_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );
        let err = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();

        // Then we expect an error.

        assert_custom_error!(err, AssetError::ExtensionLengthInvalid);
    }

    #[tokio::test]
    async fn update_with_invalid_extension_data_fails() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let create_ix = CreateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey(), false)
            .owner(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .name("name".to_string())
            .instruction();

        // And we create a new asset.

        let tx = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &asset],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

        let account = context
            .banks_client
            .get_account(asset.pubkey())
            .await
            .unwrap();

        assert!(account.is_some());
        let account = account.unwrap();

        let account_data = account.data.as_ref();
        assert!(Asset::get_extensions(account_data).is_empty());

        let mut attributes = AttributesBuilder::default();
        attributes.add("hat", "nifty");
        let data = attributes.data();

        let update_ix = UpdateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .extension(ExtensionInput {
                extension_type: ExtensionType::Attributes,
                // increase the data length
                length: data.len() as u32 + 100,
                data: Some(data),
            })
            .instruction();

        // When we update the asset with an invalid extension length.
        let tx = Transaction::new_signed_with_payer(
            &[update_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );
        let err = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();

        // Then we expect an error.

        assert_custom_error!(err, AssetError::ExtensionDataInvalid);
    }
}
