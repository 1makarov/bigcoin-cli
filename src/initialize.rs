use super::{BigcoinAbi, CHAIN_ID, CONTRACT, provider_ext::ProviderExt};
use alloy::{
    eips::eip1559::Eip1559Estimation,
    network::{Network, NetworkWallet, TransactionBuilder},
    primitives::U256,
    providers::{Provider, RootProvider},
    sol_types::SolCall,
};
use tokio::task::JoinSet;

pub async fn multi_initialize<N: Network, W: NetworkWallet<N> + 'static>(
    provider: RootProvider<N>,
    wallets: Vec<W>,
    max_threads: usize,
) {
    let mut join_set = JoinSet::new();
    let mut iter = wallets.into_iter();

    for _ in 0..max_threads {
        if let Some(wallet) = iter.next() {
            let provider = provider.clone();
            join_set.spawn(initialize(provider, wallet));
        }
    }

    while let Some(task) = join_set.join_next().await {
        if let Err(e) = task.expect("join") {
            println!("{e:?}");
        }

        if let Some(wallet) = iter.next() {
            let provider = provider.clone();
            join_set.spawn(initialize(provider, wallet));
        }
    }
}

pub async fn initialize<N: Network, W: NetworkWallet<N>>(
    provider: RootProvider<N>,
    wallet: W,
) -> anyhow::Result<()> {
    let addr = wallet.default_signer_address();

    let initialized = provider
        .call_decode::<bool>(
            N::TransactionRequest::default()
                .with_to(CONTRACT)
                .with_input(BigcoinAbi::initializedStarterFacilityCall(addr).abi_encode()),
        )
        .await?;

    if initialized {
        println!("[{addr}] already initialized");

        return Ok(());
    }

    let init_price = provider
        .call_decode::<U256>(
            N::TransactionRequest::default()
                .with_to(CONTRACT)
                .with_input(BigcoinAbi::initialFacilityPriceCall {}.abi_encode()),
        )
        .await?;

    let balance = provider.get_balance(addr).await?;
    if init_price > balance {
        println!(
            "[{addr}] balance is not enough: {}, init price: {}",
            balance, init_price
        );

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
        .with_to(CONTRACT)
        .with_value(init_price)
        .with_chain_id(CHAIN_ID)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(max_priority_fee_per_gas)
        .with_nonce(nonce)
        .with_input([
            62, 137, 187, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 231, 145, 197, 232, 222, 104,
            112, 68, 176, 31, 251, 40, 24, 139, 125, 39, 235, 24, 224, 232,
        ]);

    let gas_limit = provider.estimate_gas(tx.clone()).await?;
    tx.set_gas_limit(gas_limit);

    let fee = U256::from(max_fee_per_gas + max_priority_fee_per_gas) * U256::from(gas_limit);
    if fee + init_price > balance {
        println!(
            "[{addr}] balance is not enough to pay fee + init_price: {}",
            fee + init_price
        );

        return Ok(());
    }

    let raw = tx.build(&wallet).await?;
    let tx_hash = *provider.send_tx_envelope(raw).await?.tx_hash();
    println!("[{addr}] transaction sent: {tx_hash}");

    Ok(())
}
