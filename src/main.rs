mod cli;
mod commands;
mod config;
mod error;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ConfigCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::run(args).await?,
        Commands::Node => commands::node::run().await?,
        Commands::Faucet(args) => commands::faucet::run(args).await?,
        Commands::Status => commands::status::run().await?,
        Commands::Logs { follow } => commands::logs::run(follow).await?,
        Commands::Dev => commands::dev::run().await?,
        Commands::Build => commands::build::run().await?,
        Commands::Upload => commands::upload::run().await?,
        Commands::Deploy => commands::deploy::run().await?,
        Commands::Inspect(args) => commands::inspect::run(args).await?,
        Commands::Doctor => commands::doctor::run().await?,
        Commands::Config(config_cmd) => match config_cmd {
            ConfigCommand::Init => commands::config::init().await?,
            ConfigCommand::Set(args) => commands::config::set(args).await?,
            ConfigCommand::Show => commands::config::show().await?,
        },
        Commands::ChainInfo => commands::chain_info::run().await?,
    }

    Ok(())
}
