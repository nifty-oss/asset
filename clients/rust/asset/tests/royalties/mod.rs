#![cfg(feature = "test-sbf")]

use nifty_asset::{
    constraints::{Account, OperatorType, OwnedByBuilder},
    extensions::{ExtensionBuilder, Royalties, RoyaltiesBuilder},
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

    let mut owned_by = OwnedByBuilder::default();
    owned_by.set(Account::Recipient, &[system_program::ID]);

    let mut royalties = RoyaltiesBuilder::default();
    royalties.set(500, &mut owned_by);
    let data = royalties.data();

    // When we create a new asset.

    let ix = CreateBuilder::new()
        .asset(asset.pubkey())
        .authority(context.payer.pubkey(), false)
        .owner(context.payer.pubkey())
        .payer(Some(context.payer.pubkey()))
        .system_program(Some(system_program::id()))
        .name("name".to_string())
        .extensions(vec![ExtensionInput {
            extension_type: ExtensionType::Royalties,
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
    let royalties = Asset::get::<Royalties>(account_data).unwrap();

    assert_eq!(*royalties.basis_points, 500);
    assert_eq!(
        royalties.constraint.operator.operator_type(),
        OperatorType::OwnedBy
    );
}
