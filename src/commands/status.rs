use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;
use std::path::Path;
use std::process::Command;

use subxt::{OnlineClient, PolkadotConfig};

pub async fn run() -> Result<()> {
    println!();
    println!("{}", "⚡ PotKit System Status".bold().green());
    println!();

    // ---------------------------------------------------
    // Environment
    // ---------------------------------------------------

    println!("{}", "Environment".bold().cyan());

    check_command("rustc", "Rust");
    check_command("cargo", "Cargo");
    check_command("docker", "Docker");

    println!();

    // ---------------------------------------------------
    // Config
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

    let network = value
        .get("network")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let default_account = value
        .get("default_account")
        .and_then(|v| v.as_str())
        .unwrap_or("not configured");

    // ---------------------------------------------------
    // Blockchain
    // ---------------------------------------------------

    println!("{}", "Blockchain".bold().cyan());

    match OnlineClient::<PolkadotConfig>::from_url(rpc_url).await {
        Ok(api) => {
            println!("  RPC Connected    : {}", "Yes".green());

            let latest = api.blocks().at_latest().await?;
            let header = latest.header();

            println!(
                "  Latest Block     : {}",
                header.number.to_string().yellow()
            );
        }

        Err(_) => {
            println!("  RPC Connected    : {}", "No".red());
        }
    }

    println!();

    // ---------------------------------------------------
    // Configuration
    // ---------------------------------------------------

    println!("{}", "Configuration".bold().cyan());

    println!("  Network          : {}", network.yellow());

    println!("  RPC URL          : {}", rpc_url.yellow());

    println!("  Default Account  : {}", default_account.yellow());

    println!();

    // ---------------------------------------------------
    // Project Artifacts
    // ---------------------------------------------------

    println!("{}", "Project".bold().cyan());

    let ink_dir = Path::new("target").join("ink");

    if ink_dir.exists() {
        println!("  target/ink       : {}", "Found".green());

        let wasm_found = has_extension(&ink_dir, "wasm");
        let contract_found = has_extension(&ink_dir, "contract");

        println!(
            "  .wasm            : {}",
            if wasm_found {
                "Found".green()
            } else {
                "Missing".red()
            }
        );

        println!(
            "  .contract        : {}",
            if contract_found {
                "Found".green()
            } else {
                "Missing".red()
            }
        );
    } else {
        println!("  target/ink       : {}", "Missing".red());
    }

    println!();

    println!("{}", "🚀 PotKit environment operational.".bold().green());

    Ok(())
}

fn check_command(cmd: &str, label: &str) {
    let result = Command::new(cmd).arg("--version").output();

    match result {
        Ok(output) if output.status.success() => {
            println!("  {:<17}: {}", label, "OK".green());
        }

        _ => {
            println!("  {:<17}: {}", label, "Missing".red());
        }
    }
}

fn has_extension(dir: &Path, ext: &str) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) == Some(ext) {
                return true;
            }
        }
    }

    false
}
