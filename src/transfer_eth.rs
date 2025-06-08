use alloy::{
    eips::eip1559::Eip1559Estimation,
    network::{Network, NetworkWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{Provider, RootProvider},
};
use clap::Parser;
use tokio::task::JoinSet;

use crate::CHAIN_ID;

#[derive(Debug, Clone, Copy, Parser)]
pub struct TransferEthParams {
    /// Recipient address for eth
    #[clap(short, long)]
    pub receiver: Address,
}

pub async fn transfer_eth<N: Network, W: NetworkWallet<N> + 'static>(
    provider: RootProvider<N>,
    wallets: Vec<W>,
    params: TransferEthParams,
    max_threads: usize,
) {
    let mut join_set = JoinSet::new();
    let mut iter = wallets.into_iter();

    for _ in 0..max_threads {
        if let Some(wallet) = iter.next() {
            let provider = provider.clone();
            join_set.spawn(transfer(provider, wallet, params));
        }
    }

    while let Some(task) = join_set.join_next().await {
        if let Err(e) = task.expect("join") {
            println!("{e:?}");
        }

        if let Some(wallet) = iter.next() {
            let provider = provider.clone();
            join_set.spawn(transfer(provider, wallet, params));
        }
    }
}

pub async fn transfer<N: Network, W: NetworkWallet<N>>(
    provider: RootProvider<N>,
    wallet: W,
    TransferEthParams { receiver }: TransferEthParams,
) -> anyhow::Result<()> {
    let addr = wallet.default_signer_address();
    if addr == receiver {
        return Ok(());
    }

    println!("[{addr}] processing...");
    let nonce = provider.get_transaction_count(addr).await?;
    let Eip1559Estimation {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    } = provider.estimate_eip1559_fees().await?;

    let mut tx = N::TransactionRequest::default()
        .with_from(addr)
        .with_to(receiver)
        .with_chain_id(CHAIN_ID)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(max_priority_fee_per_gas)
        .with_nonce(nonce);

    let gas_limit = provider.estimate_gas(tx.clone()).await?;
    tx.set_gas_limit(gas_limit);

    let fee = U256::from(max_fee_per_gas + max_priority_fee_per_gas)
        * U256::from(gas_limit);
    let mut balance = provider.get_balance(addr).await?;

    if fee > balance {
        println!("[{addr}] no enough balance to pay fee: {}", fee);

        return Ok(());
    } else {
        balance = balance - fee;
    }
    tx.set_value(balance);

    let raw = tx.build(&wallet).await?;
    let tx_hash = *provider.send_tx_envelope(raw).await?.tx_hash();
    println!("[{addr}] transaction sent: {tx_hash}");

    Ok(())
}
