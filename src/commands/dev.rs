use anyhow::{bail, Context, Result};
use colored::*;
use dirs::home_dir;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use tokio::time::{sleep, Duration};

use subxt::{OnlineClient, PolkadotConfig};

pub async fn run() -> Result<()> {
    println!();
    println!("{}", "🚀 Starting PotKit Dev Environment...".bold().green());
    println!();

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
    // Blockchain Connection
    // ---------------------------------------------------

    let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;

    println!("{} {}", "✓ RPC connected:".green(), rpc_url.yellow());

    // ---------------------------------------------------
    // Contract Artifacts
    // ---------------------------------------------------

    let ink_dir = Path::new("target").join("ink");

    if ink_dir.exists() {
        println!("{}", "✓ Contract artifacts detected".green());
    } else {
        println!("{}", "⚠ No contract artifacts found".yellow());
    }

    // ---------------------------------------------------
    // File Watcher
    // ---------------------------------------------------

    let watch_path: PathBuf = if Path::new("src").exists() {
        PathBuf::from("src")
    } else if Path::new("Cargo.toml").exists() || Path::new("lib.rs").exists() {
        PathBuf::from(".")
    } else {
        bail!(
            "No source directory found to watch. Run `potkit dev` from a project root or create a src/ directory."
        );
    };

    let (tx, rx) = channel();

    let mut watcher = recommended_watcher(tx)?;

    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

    println!(
        "{} {}",
        "✓ Watching".green(),
        watch_path.display().to_string().yellow()
    );

    // ---------------------------------------------------
    // Runtime
    // ---------------------------------------------------

    println!("{}", "✓ Live chain monitor active".green());

    println!();
    println!("{}", "🔥 PotKit dev mode running".bold().cyan());
    println!();

    let mut last_block = 0u64;

    loop {
        // ---------------------------------------------------
        // Chain Monitor
        // ---------------------------------------------------

        if let Ok(latest) = api.blocks().at_latest().await {
            let header = latest.header();

            let current = header.number as u64;

            if current != last_block {
                println!(
                    "{} {}",
                    "[chain]".bright_blue(),
                    format!("Block #{}", current).green()
                );

                last_block = current;
            }
        }

        // ---------------------------------------------------
        // File Watch Events
        // ---------------------------------------------------

        match rx.try_recv() {
            Ok(event) => {
                println!("{} {:?}", "[watch]".bright_yellow(), event);

                println!("{}", "[build] Rebuild recommended".bright_green());
            }

            Err(_) => {}
        }

        sleep(Duration::from_secs(2)).await;
    }
}
