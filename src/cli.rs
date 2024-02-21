use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Miden Faucet")]
#[clap(about = "A command line tool for Miden Faucet", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Starts the server
    Start {
        #[clap(long, required = true)]
        token_symbol: String,

        #[clap(long, required = true)]
        decimals: u32,

        #[clap(long, required = true)]
        max_supply: u64,
    },
}
