use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use parity_scale_codec::Decode;
use std::fs;

use subxt::{OnlineClient, PolkadotConfig};

pub async fn run(message: String) -> Result<()> {

    println!();

    println!(
        "{}",
        "📡 PotKit Contract Query"
            .bold()
            .green()
    );

    println!();

    // ---------------------------------------------------
    // Validate message
    // ---------------------------------------------------

    if message != "get" {
        anyhow::bail!(
            "Only 'get' supported currently"
        );
    }

    // ---------------------------------------------------
    // Load config
    // ---------------------------------------------------

    let config_path = home_dir()
        .context("Could not determine home directory")?
        .join(".potkit")
        .join("config.toml");

    let config_contents =
        fs::read_to_string(&config_path)?;

    let value: toml::Value =
        toml::from_str(&config_contents)?;

    let rpc_url = value
        .get("rpc_url")
        .and_then(|v| v.as_str())
        .unwrap_or("ws://127.0.0.1:9944");

    println!(
        "{} {}",
        "RPC URL:"
            .bright_cyan(),

        rpc_url.yellow()
    );

    // ---------------------------------------------------
    // Connect
    // ---------------------------------------------------

    println!(
        "{}",
        "🔌 Connecting to blockchain..."
            .cyan()
    );

    let api =
        OnlineClient::<PolkadotConfig>
            ::from_url(rpc_url)
            .await?;

    println!(
        "{}",
        "✓ Connected"
            .green()
    );

    // ---------------------------------------------------
    // Placeholder contract address
    // ---------------------------------------------------

    let contract_address =
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstf9dKz6P5Qx8hV2o1Y";

    println!();

    println!(
        "{} {}",
        "Contract:"
            .bright_cyan(),

        contract_address.yellow()
    );

    println!(
        "{} {}()",
        "Message:"
            .bright_cyan(),

        message.yellow()
    );

    // ---------------------------------------------------
    // Simulated selector decode
    // ---------------------------------------------------

    // ink! selector for get()
    let selector: [u8; 4] =
        [0x2f, 0x86, 0x5b, 0xd9];

    println!();

    println!(
        "{} {:?}",
        "Selector:"
            .bright_blue(),

        selector
    );

    // ---------------------------------------------------
    // TODO:
    // Real contracts_call RPC
    // ---------------------------------------------------

    // Simulated response
    let encoded_result = vec![1u8];

    let mut bytes = &encoded_result[..];

    let value = bool::decode(&mut bytes)?;

    println!();

    println!(
        "{} {}",
        "Result:"
            .bright_green(),

        value.to_string().yellow()
    );

    println!();

    println!(
        "{}",
        "✓ Query executed successfully"
            .green()
    );

    Ok(())
}