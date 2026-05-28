use crate::utils::output::tx_event;
use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;

use subxt::utils::AccountId32;
use subxt::{dynamic, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

use crate::cli::FaucetArgs;

pub async fn run(args: FaucetArgs) -> Result<()> {
    println!(
        "{}",
        format!("💧 Sending test tokens to {}", args.address)
            .bold()
            .green()
    );

    // Load RPC URL from config
    let config_path = home_dir()
        .context("Could not determine home directory")?
        .join(".potkit")
        .join("config.toml");

    let config_contents = fs::read_to_string(&config_path).context("Failed to read config.toml")?;

    let value: toml::Value =
        toml::from_str(&config_contents).context("Failed to parse config.toml")?;

    let rpc_url = value
        .get("rpc_url")
        .and_then(|v| v.as_str())
        .unwrap_or("ws://127.0.0.1:9944");

    println!("RPC URL: {}", rpc_url.yellow());

    // Connect to node
    println!("{}", "🔌 Connecting to node...".cyan());
    let api: OnlineClient<PolkadotConfig> = OnlineClient::from_url(rpc_url)
        .await
        .context("Failed to connect to Portaldot node. Run `potkit node` first!")?;

    let signer = dev::alice();

    let dest: AccountId32 = args
        .address
        .parse()
        .context("Invalid destination address")?;

    let amount: u128 = 1_000_000_000_000;

    println!("{}", "⏳ Submitting transaction...".cyan());

    // Use dynamic API to construct the transfer_allow_death transaction
    let tx_payload = dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![
            dynamic::Value::unnamed_variant(
                "Id",
                vec![dynamic::Value::from_bytes(dest.0.to_vec())],
            ),
            dynamic::Value::u128(amount),
        ],
    );

    let tx_progress = api
        .tx()
        .sign_and_submit_then_watch_default(&tx_payload, &signer)
        .await?;

    let tx_in_block = tx_progress.wait_for_finalized().await?;

    tx_event("Faucet transfer finalized");
    println!();
    println!("{}", "✅ Transaction Successful!".bold().green());
    println!(
        "   Block Hash : {}",
        tx_in_block.block_hash().to_string().yellow()
    );
    println!("   Amount     : 1 PORTAL");
    println!("   From       : Alice");
    println!("   To         : {}", args.address);

    Ok(())
}
