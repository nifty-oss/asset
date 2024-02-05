#![cfg(feature = "test-sbf")]

use nifty_asset::{
    extensions::{Attributes, AttributesBuilder, ExtensionBuilder},
    instructions::{AllocateBuilder, CreateBuilder},
    state::{Asset, Discriminator, Standard, State},
    types::{Extension, ExtensionType},
    ZeroCopy,
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

        let account_data = account.data.as_ref();
        let asset = Asset::load(account_data);

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, Standard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.holder, context.payer.pubkey());
        // we are not expecting any extension on the account
        assert!(Asset::get_extensions(account_data).is_empty());
    }

    #[tokio::test]
    async fn create_with_extension() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        // And an extension.

        let mut attributes = AttributesBuilder::default();
        attributes.add("hat", "nifty");
        let data = attributes.build();

        let ix = AllocateBuilder::new()
            .asset(asset.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .extension(Extension {
                extension_type: ExtensionType::Attributes,
                length: data.len() as u32,
                data: Some(data),
            })
            .instruction();

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &asset],
            context.last_blockhash,
        );
        context.banks_client.process_transaction(tx).await.unwrap();

        // When we create a new asset.

        let ix = CreateBuilder::new()
            .asset(asset.pubkey())
            .authority(context.payer.pubkey())
            .holder(context.payer.pubkey())
            .payer(Some(context.payer.pubkey()))
            .system_program(Some(system_program::id()))
            .name("name".to_string())
            .instruction();

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

        let account_data = account.data.as_ref();
        let asset = Asset::load(account_data);

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, Standard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.holder, context.payer.pubkey());

        // And we are expecting an extension on the account.

        assert!(Asset::get_extensions(account_data).len() == 1);
        let attributes = Asset::get::<Attributes>(account_data).unwrap();

        assert_eq!(attributes.traits.len(), 1);
        assert_eq!(attributes.traits[0].name.as_str(), "hat");
        assert_eq!(attributes.traits[0].value.as_str(), "nifty");
    }
}
