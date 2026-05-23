use anyhow::{bail, Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};
use subxt::{dynamic, OnlineClient, PolkadotConfig};

use subxt_signer::sr25519::dev;

pub async fn run() -> Result<()> {
    println!("{}", "📤 Uploading smart contract code...".bold().green());

    // Locate target/ink
    let ink_dir = Path::new("target").join("ink");

    if !ink_dir.exists() {
        bail!("target/ink not found. Run `potkit build` first.");
    }

    // Find wasm file
    let wasm_file = find_wasm_file(&ink_dir)?.context("No .wasm file found in target/ink")?;

    println!(
        "{} {}",
        "✓ Wasm artifact found:".green(),
        wasm_file.display().to_string().yellow()
    );

    // Read wasm bytes
    let wasm_bytes = fs::read(&wasm_file)?;

    println!(
        "{} {} bytes",
        "✓ Wasm size:".green(),
        wasm_bytes.len().to_string().yellow()
    );

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
    let connect_spinner = ProgressBar::new_spinner();

    connect_spinner.set_style(ProgressStyle::with_template("{spinner:.green} {msg}")?);

    connect_spinner.enable_steady_tick(std::time::Duration::from_millis(120));

    connect_spinner.set_message("Connecting to blockchain...");

    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;

    connect_spinner.finish_with_message("Connected to blockchain");

    // Alice signer
    let signer = dev::alice();

    let upload_spinner = ProgressBar::new_spinner();

    upload_spinner.set_style(ProgressStyle::with_template("{spinner:.cyan} {msg}")?);

    upload_spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    upload_spinner.set_message("Uploading Wasm code...");

    // Build dynamic upload_code extrinsic
    let tx = dynamic::tx(
        "Contracts",
        "upload_code",
        vec![
            dynamic::Value::from_bytes(&wasm_bytes),
            dynamic::Value::named_variant("None", Vec::<(String, dynamic::Value<()>)>::new()),
            dynamic::Value::named_variant("Enforced", Vec::<(String, dynamic::Value<()>)>::new()),
        ],
    );

    // Submit transaction
    let tx_progress = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?;

    let tx_in_block = tx_progress.wait_for_finalized().await?;

    upload_spinner.finish_with_message("Wasm upload finalized");

    println!();
    println!("{}", "✅ Contract Code Uploaded!".bold().green());

    println!(
        "  Block Hash : {}",
        tx_in_block.block_hash().to_string().yellow()
    );

    println!(
        "  Wasm Size  : {} bytes",
        wasm_bytes.len().to_string().yellow()
    );

    println!("  Signer     : {}", "Alice".yellow());

    println!();
    println!(
        "{}",
        "🚀 Wasm bytecode stored on-chain successfully."
            .bold()
            .green()
    );

    Ok(())
}

fn find_wasm_file(dir: &Path) -> Result<Option<PathBuf>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
            return Ok(Some(path));
        }
    }

    Ok(None)
}
