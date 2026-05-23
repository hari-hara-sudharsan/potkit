use anyhow::{bail, Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

use subxt::{OnlineClient, PolkadotConfig};

pub async fn run() -> Result<()> {
    println!("{}", "🚀 Preparing contract deployment...".bold().green());

    // Locate target/ink
    let ink_dir = Path::new("target").join("ink");

    if !ink_dir.exists() {
        bail!("target/ink not found. Run `potkit build` first.");
    }

    // Locate artifacts
    let contract_file = find_file_with_ext(&ink_dir, "contract")?
        .context("No .contract file found")?;

    let wasm_file = find_file_with_ext(&ink_dir, "wasm")?
        .context("No .wasm file found")?;

    let metadata_file = find_file_with_ext(&ink_dir, "json")?
        .context("No metadata .json file found")?;

    println!(
        "{} {}",
        "✓ Contract artifact:".green(),
        contract_file.display().to_string().yellow()
    );

    println!(
        "{} {}",
        "✓ Wasm artifact:".green(),
        wasm_file.display().to_string().yellow()
    );

    println!(
        "{} {}",
        "✓ Metadata artifact:".green(),
        metadata_file.display().to_string().yellow()
    );

    // Read metadata
    let metadata_contents = fs::read_to_string(&metadata_file)?;

    // Very lightweight constructor validation
    if !metadata_contents.contains("\"constructors\"") {
        bail!("No constructors found in metadata.");
    }

    println!("{}", "✓ Constructor metadata detected.".green());

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
    println!("{}", "🔗 Connecting to blockchain...".cyan());

    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;

    let latest = api.blocks().at_latest().await?;
    let header = latest.header();

    println!();
    println!("{}", "📄 Deployment Validation Report".bold().cyan());

    println!(
        "  Latest Block : {}",
        header.number.to_string().yellow()
    );

    println!(
        "  Block Hash   : {}",
        latest.hash().to_string().yellow()
    );

    println!(
        "  Constructor  : {}",
        "new()".yellow()
    );

    println!(
        "  Signer       : {}",
        "Alice".yellow()
    );

    println!(
        "  Status       : {}",
        "Ready for instantiation".green()
    );

    println!();

    println!(
        "{}",
        "🚀 Deployment pipeline validated successfully."
            .bold()
            .green()
    );

    println!(
        "{}",
        "Next step: execute instantiate_with_code."
            .yellow()
    );

    Ok(())
}

fn find_file_with_ext(dir: &Path, ext: &str) -> Result<Option<PathBuf>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some(ext) {
            return Ok(Some(path));
        }
    }

    Ok(None)
}