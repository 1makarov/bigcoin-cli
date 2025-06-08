use std::path::PathBuf;

use bigcoin_cli::{
    add_starter::AddStarterParams, claim::ClaimParams, transfer::TransferParams,
    transfer_eth::TransferEthParams,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    /// Max concurrent threads.
    #[clap(short, long, default_value_t = 20)]
    pub max_threads: usize,

    /// Action to perform (e.g. initialize, claim, transfer, etc.)
    #[clap(subcommand)]
    pub action: Action,

    /// Path to the file with private keys
    #[clap(short, long)]
    pub path: PathBuf,

    /// RPC endpoint
    #[clap(long, default_value = "https://api.mainnet.abs.xyz")]
    pub rpc: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    /// Initial payment
    #[clap(visible_alias = "init")]
    Initialize,

    /// Add starter miner
    #[clap(visible_alias = "start")]
    AddStarter(AddStarterParams),

    /// Claims available rewards
    Claim(ClaimParams),

    /// Transfer tokens to address.
    #[clap(visible_alias = "send")]
    Transfer(TransferParams),

    /// Transfer eth to address
    #[clap(visible_alias = "eth")]
    TransferEth(TransferEthParams),

    /// Print total rewards
    Print,
}
