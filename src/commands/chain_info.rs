use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;
use subxt::OnlineClient;
use subxt::PolkadotConfig;

pub async fn run() -> Result<()> {
    println!("{}", "🔗 Connecting to Portaldot...".bold().green());

    // Load config file
    let config_path = home_dir()
        .context("Could not determine home directory")?
        .join(".potkit")
        .join("config.toml");

    let config_contents = fs::read_to_string(&config_path)
        .with_context(|| format!("Could not read {}", config_path.display()))?;

    // Extract rpc_url from config.toml
    let value: toml::Value = toml::from_str(&config_contents)?;
    let rpc_url = value
        .get("rpc_url")
        .and_then(|v| v.as_str())
        .unwrap_or("ws://127.0.0.1:9944");

    println!("RPC URL: {}", rpc_url.yellow());

    // Connect to chain
    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;

    // Fetch latest block
    let latest = api.blocks().at_latest().await?;
    let header = latest.header();

    println!();
    println!("{}", "📦 Chain Information".bold().cyan());
    println!("  Block Number : {}", header.number.to_string().yellow());
    println!("  Block Hash   : {}", latest.hash().to_string().yellow());

    println!();
    println!("{}", "✓ Connected to blockchain successfully.".green());

    Ok(())
}
