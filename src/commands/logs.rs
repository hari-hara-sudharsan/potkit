use crate::utils::output::print_chain_event;
use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;
use tokio::time::{sleep, Duration};

use subxt::{OnlineClient, PolkadotConfig};

pub async fn run(follow: bool) -> Result<()> {
    println!("{}", "📡 PotKit Event Stream".bold().green());

    // Load config
    let config_path = home_dir()
        .context("Could not determine home directory")?
        .join(".potkit")
        .join("config.toml");

    let config_contents = fs::read_to_string(&config_path)?;
    let value: toml::Value = toml::from_str(&config_contents)?;

    let rpc_url = value
        .get("rpc_url")
        .and_then(|v| v.as_str())
        .unwrap_or("ws://127.0.0.1:9944");

    println!("RPC URL: {}", rpc_url.yellow());

    // Connect
    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;

    println!("{}", "✅ Connected successfully.".green());

    // Single-shot mode
    if !follow {
        let latest = api.blocks().at_latest().await?;
        let header = latest.header();

        println!();
        print_chain_event(&format!("New Block #{}", header.number));

        return Ok(());
    }

    println!();
    println!("{}", "🚀 Following live blocks...".bold().cyan());
    println!();

    let mut last_seen = 0u64;

    loop {
        let latest = api.blocks().at_latest().await?;
        let header = latest.header();

        let current = header.number as u64;

        if current != last_seen {
            print_chain_event(&format!("New Block #{}", current));

            last_seen = current;
        }

        sleep(Duration::from_secs(2)).await;
    }
}
