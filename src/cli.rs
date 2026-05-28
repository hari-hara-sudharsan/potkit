use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "potkit",
    version,
    about = "The all-in-one Portaldot developer CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init(InitArgs),
    Node,
    Faucet(FaucetArgs),
    Status,
    Logs {
        #[arg(long)]
        follow: bool,
    },
    Dev,
    Build,
    Upload,
    Instantiate,
    Deploy,
    Inspect(AddressArgs),
    AbiInspect,
    Query {
        message: String,
    },
    Call {
        message: String,
    },
    Doctor,
    #[command(subcommand)]
    Config(ConfigCommand),
    ChainInfo,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    Init,
    Set(ConfigSetArgs),
    Show,
}

#[derive(Args)]
pub struct ConfigSetArgs {
    pub key: String,
    pub value: String,
}

#[derive(Args)]
pub struct InitArgs {
    pub project_name: String,
}

#[derive(Args)]
pub struct FaucetArgs {
    pub address: String,
}

#[derive(Args)]
pub struct AddressArgs {
    pub address: String,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}