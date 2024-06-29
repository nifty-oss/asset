#![cfg(feature = "test-sbf")]

use nifty_asset::{
    extensions::{
        AttributesBuilder, ExtensionBuilder, MetadataBuilder, Properties, PropertiesBuilder,
    },
    instructions::CreateBuilder,
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

#[tokio::test]
async fn create() {
    let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
        .start_with_context()
        .await;

    // Given a new keypair.

    let asset = Keypair::new();

    // And an extension.

    let mut properties = PropertiesBuilder::default();
    properties.add::<&str>("name", "nifty");
    properties.add::<u64>("points", 0);
    let data = properties.data();

    // When we create a new asset.

    let ix = CreateBuilder::new()
        .asset(asset.pubkey())
        .authority(context.payer.pubkey(), false)
        .owner(context.payer.pubkey())
        .payer(Some(context.payer.pubkey()))
        .system_program(Some(system_program::id()))
        .name("name".to_string())
        .extensions(vec![ExtensionInput {
            extension_type: ExtensionType::Properties,
            length: data.len() as u32,
            data: Some(data),
        }])
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
    assert_eq!(asset.owner, context.payer.pubkey());

    // And we are expecting an extension on the account.

    assert!(Asset::get_extensions(account_data).len() == 1);
    let properties = Asset::get::<Properties>(account_data).unwrap();

    assert_eq!(properties.len(), 2);
    assert_eq!(properties[0].name.as_str(), "name");
    assert_eq!(properties[1].name.as_str(), "points");
}

#[tokio::test]
async fn create_with_multiple() {
    let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
        .start_with_context()
        .await;

    // Given a new keypair.

    let asset = Keypair::new();

    // And multiple extensions.

    let mut metadata = MetadataBuilder::default();
    metadata.set(Some("NIFTY"), None, None, None);

    let mut properties = PropertiesBuilder::default();
    properties.add::<&str>("name", "nifty");
    properties.add::<u64>("points", 0);
    properties.add::<bool>("active", true);

    let mut attributes = AttributesBuilder::default();
    attributes.add("type", "solid");

    // When we create a new asset.

    let ix = CreateBuilder::new()
        .asset(asset.pubkey())
        .authority(context.payer.pubkey(), false)
        .owner(context.payer.pubkey())
        .payer(Some(context.payer.pubkey()))
        .system_program(Some(system_program::id()))
        .name("Asset with extensions".to_string())
        .extensions(vec![
            ExtensionInput {
                extension_type: ExtensionType::Metadata,
                length: metadata.len() as u32,
                data: Some(metadata.data()),
            },
            ExtensionInput {
                extension_type: ExtensionType::Properties,
                length: properties.len() as u32,
                data: Some(properties.data()),
            },
            ExtensionInput {
                extension_type: ExtensionType::Attributes,
                length: attributes.len() as u32,
                data: Some(attributes.data()),
            },
        ])
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
    assert_eq!(asset.owner, context.payer.pubkey());

    // And we are expecting an extension on the account.

    assert!(Asset::get_extensions(account_data).len() == 3);
    let properties = Asset::get::<Properties>(account_data).unwrap();

    assert_eq!(properties.len(), 3);
    assert_eq!(properties[0].name.as_str(), "name");
    assert_eq!(properties[1].name.as_str(), "points");
    assert_eq!(properties[2].name.as_str(), "active");
}
