use crate::utils::output::{build_event, chain_event, watch_event};
use anyhow::{anyhow, bail, Context, Result};
use colored::*;
use dirs::home_dir;
use notify::{RecursiveMode, Watcher};
use std::env;
use std::fs;
use std::path::Path;
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

    let mut watch_path = env::current_dir()?.join("src");

    // If src doesn't exist in current dir, check parent
    if !watch_path.exists() {
        watch_path = env::current_dir()?
            .parent()
            .ok_or_else(|| anyhow!("Could not find parent directory"))?
            .join("src");
    }

    // Final check - make sure the path exists
    if !watch_path.exists() {
        bail!(
            "Could not find 'src' directory. Current dir: {}\nTried: {}",
            env::current_dir()?.display(),
            watch_path.display()
        );
    }

    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        tx.send(res).unwrap();
    })?;

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
                chain_event(&format!("New Block #{}", current));

                last_block = current;
            }
        }

        // ---------------------------------------------------
        // File Watch Events
        // ---------------------------------------------------

        match rx.try_recv() {
            Ok(Ok(event)) => {
                watch_event("Contract source modified");
                println!("{} {:?}", "[watch]".bright_yellow(), event.paths);

                build_event("Rebuild recommended");
            }

            Ok(Err(e)) => {
                println!("{} {:?}", "[watch-error]".red(), e);
            }

            Err(_) => {}
        }

        sleep(Duration::from_secs(2)).await;
    }
}
