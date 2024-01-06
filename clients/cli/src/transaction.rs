use anyhow::Result;
use retry::{delay::Exponential, retry};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::Instruction;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

#[macro_export]
macro_rules! transaction {
    ($signers:expr, $instructions:expr, $client:expr) => {
        Transaction::new_signed_with_payer(
            $instructions,
            Some(&$signers[0].pubkey()),
            $signers,
            $client.get_latest_blockhash()?,
        )
    };
}

pub fn send_and_confirm_tx(
    client: &RpcClient,
    signers: &[&Keypair],
    ixs: &[Instruction],
) -> Result<Signature> {
    let tx = transaction!(signers, ixs, client);

    let signature = client.send_and_confirm_transaction(&tx)?;

    Ok(signature)
}

pub fn send_and_confirm_tx_with_retries(
    client: &RpcClient,
    signers: &[&Keypair],
    ixs: &[Instruction],
) -> Result<Signature> {
    let tx = transaction!(signers, ixs, client);

    // Send tx with retries.
    let res = retry(
        Exponential::from_millis_with_factor(250, 2.0).take(3),
        || client.send_and_confirm_transaction_with_spinner(&tx),
    )?;

    Ok(res)
}
