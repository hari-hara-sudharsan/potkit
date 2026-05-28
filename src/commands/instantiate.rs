use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;
use std::path::Path;
use tokio::process::Command;

use subxt::{OnlineClient, PolkadotConfig};

use crate::utils::output::deploy_event;
use crate::utils::spinner::create_spinner;

pub async fn run() -> Result<()> {
    println!();
    println!("{}", "📦 PotKit Contract Instantiation".bold().green());
    println!();

    // ---------------------------------------------------
    // Locate Contract Metadata
    // ---------------------------------------------------
    let contract_path = Path::new("target")
        .join("ink")
        .join("real_contract.contract");

    if !contract_path.exists() {
        anyhow::bail!("Contract metadata not found");
    }

    println!(
        "{} {}",
        "✓ Contract metadata found:".green(),
        contract_path.display()
    );

    // ---------------------------------------------------
    // Load Config
    // ---------------------------------------------------
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

    // ---------------------------------------------------
    // Connect
    // ---------------------------------------------------
    let spinner = create_spinner("Connecting to blockchain...")?;
    let _api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;
    spinner.finish_with_message("✓ Connected to blockchain");

    // ---------------------------------------------------
    // Instantiate via cargo-contract (Real Command)
    // ---------------------------------------------------
    let instantiate_spinner = create_spinner("Instantiating smart contract...")?;

    let output = Command::new("cargo")
        .args([
            "contract",
            "instantiate",
            "real_contract.contract",
            "--constructor",
            "new",
            "--suri",
            "//Alice",
            "--url",
            rpc_url,
            "--skip-confirm",
        ])
        .current_dir(".")
        .output()
        .await?;

    // ==================== SUCCESS CHECK ====================
    if output.status.success() {
        instantiate_spinner.finish_with_message("✓ Contract instantiated");

        println!();
        println!("{}", String::from_utf8_lossy(&output.stdout));

        deploy_event("Smart contract deployed on-chain");
    } else {
        anyhow::bail!(String::from_utf8_lossy(&output.stderr).to_string());
    }
    // =======================================================

    println!();
    println!("{}", "🚀 Contract is now live on Portaldot".bold().green());

    Ok(())
}