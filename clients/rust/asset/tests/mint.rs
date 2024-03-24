#![cfg(feature = "test-sbf")]

use std::fs::File;

use nifty_asset::{
    extensions::ExtensionType as NiftyExtensionType,
    extensions::{Creators, Royalties},
    mint,
    state::{Asset, Discriminator, Standard as NiftyStandard, State},
    types::ExtensionType,
    types::Standard,
    AssetArgs, ExtensionArgs, ExtensionValue, JsonCreator, JsonRoyalties, MintAccounts, MintIxArgs,
    ZeroCopy,
};
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod mint {
    use super::*;

    #[tokio::test]
    async fn mint_without_extensions() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let accounts = MintAccounts {
            asset: asset.pubkey(),
            owner: context.payer.pubkey(),
            payer: None,
        };

        let asset_args = AssetArgs {
            name: "name".to_string(),
            standard: Standard::NonFungible,
            mutable: true,
        };

        let extension_args = vec![];

        let ix_args = MintIxArgs {
            accounts,
            asset_args,
            extension_args,
        };

        let ixs = mint(ix_args).unwrap();

        let tx = Transaction::new_signed_with_payer(
            &ixs,
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
        let asset = Asset::load(account_data);

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, NiftyStandard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.owner, context.payer.pubkey());
        assert!(Asset::get_extensions(account_data).is_empty());
    }

    #[tokio::test]
    async fn mint_with_creators_extension() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let accounts = MintAccounts {
            asset: asset.pubkey(),
            owner: context.payer.pubkey(),
            payer: None,
        };

        let asset_args = AssetArgs {
            name: "name".to_string(),
            standard: Standard::NonFungible,
            mutable: true,
        };

        let creators: Vec<JsonCreator> =
            serde_json::from_reader(File::open("tests/fixtures/creators.json").unwrap()).unwrap();

        let first_creator = creators.first().unwrap().clone();
        let last_creator = creators.last().unwrap().clone();

        let value = ExtensionValue::JsonCreator(creators);

        let extension_args = vec![ExtensionArgs {
            extension_type: ExtensionType::Creators,
            data: value.into_data(),
            chunked: true,
        }];

        let ix_args = MintIxArgs {
            accounts,
            asset_args,
            extension_args,
        };

        let ixs = mint(ix_args).unwrap();

        let tx = Transaction::new_signed_with_payer(
            &ixs,
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
        let asset = Asset::load(account_data);

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, NiftyStandard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.owner, context.payer.pubkey());

        // And we are expecting one extension on the account.
        let extensions = Asset::get_extensions(account_data);
        assert_eq!(extensions.len(), 1);
        // And the extension is the creators extension.
        let extension = extensions.first().unwrap();
        assert_eq!(*extension, NiftyExtensionType::Creators);

        let creators = Asset::get::<Creators>(account_data).unwrap();
        assert_eq!(
            creators.creators.first().unwrap().address,
            first_creator.address
        );
        assert_eq!(
            creators.creators.first().unwrap().share,
            first_creator.share
        );
        assert_eq!(
            creators.creators.last().unwrap().address,
            last_creator.address
        );
        assert_eq!(creators.creators.last().unwrap().share, last_creator.share);
    }

    #[tokio::test]
    async fn mint_with_royalties_allowlist() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let accounts = MintAccounts {
            asset: asset.pubkey(),
            owner: context.payer.pubkey(),
            payer: None,
        };

        let asset_args = AssetArgs {
            name: "name".to_string(),
            standard: Standard::NonFungible,
            mutable: true,
        };

        let expected_royalties: JsonRoyalties =
            serde_json::from_reader(File::open("tests/fixtures/royalties-allowlist.json").unwrap())
                .unwrap();

        let expected_basis_points = expected_royalties.basis_points;

        let value = ExtensionValue::JsonRoyalities(expected_royalties);

        let extension_args = vec![ExtensionArgs {
            extension_type: ExtensionType::Royalties,
            data: value.into_data(),
            chunked: true,
        }];

        let ix_args = MintIxArgs {
            accounts,
            asset_args,
            extension_args,
        };

        let ixs = mint(ix_args).unwrap();

        let tx = Transaction::new_signed_with_payer(
            &ixs,
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
        let asset = Asset::load(account_data);

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, NiftyStandard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.owner, context.payer.pubkey());

        // And we are expecting one extension on the account.
        let extensions = Asset::get_extensions(account_data);
        assert_eq!(extensions.len(), 1);
        // And the extension is the royalties extension.
        let extension = extensions.first().unwrap();
        assert_eq!(*extension, NiftyExtensionType::Royalties);

        let royalties = Asset::get::<Royalties>(account_data).unwrap();
        assert_eq!(royalties.basis_points, &expected_basis_points);
    }

    #[tokio::test]
    async fn mint_with_royalties_denylist() {
        let mut context = ProgramTest::new("asset_program", nifty_asset::ID, None)
            .start_with_context()
            .await;

        // Given a new keypair.

        let asset = Keypair::new();

        let accounts = MintAccounts {
            asset: asset.pubkey(),
            owner: context.payer.pubkey(),
            payer: None,
        };

        let asset_args = AssetArgs {
            name: "name".to_string(),
            standard: Standard::NonFungible,
            mutable: true,
        };

        let expected_royalties: JsonRoyalties =
            serde_json::from_reader(File::open("tests/fixtures/royalties-denylist.json").unwrap())
                .unwrap();

        let expected_basis_points = expected_royalties.basis_points;

        let value = ExtensionValue::JsonRoyalities(expected_royalties);

        let extension_args = vec![ExtensionArgs {
            extension_type: ExtensionType::Royalties,
            data: value.into_data(),
            chunked: true,
        }];

        let ix_args = MintIxArgs {
            accounts,
            asset_args,
            extension_args,
        };

        let ixs = mint(ix_args).unwrap();

        let tx = Transaction::new_signed_with_payer(
            &ixs,
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
        let asset = Asset::load(account_data);

        assert_eq!(asset.discriminator, Discriminator::Asset);
        assert_eq!(asset.state, State::Unlocked);
        assert_eq!(asset.standard, NiftyStandard::NonFungible);
        assert_eq!(asset.authority, context.payer.pubkey());
        assert_eq!(asset.owner, context.payer.pubkey());

        // And we are expecting one extension on the account.
        let extensions = Asset::get_extensions(account_data);
        assert_eq!(extensions.len(), 1);
        // And the extension is the royalties extension.
        let extension = extensions.first().unwrap();
        assert_eq!(*extension, NiftyExtensionType::Royalties);

        let royalties = Asset::get::<Royalties>(account_data).unwrap();
        assert_eq!(royalties.basis_points, &expected_basis_points);
    }
}
