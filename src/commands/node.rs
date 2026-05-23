use anyhow::{bail, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::time::Duration;

pub async fn run() -> Result<()> {
    println!("{}", "🚀 Starting local Portaldot node...".bold().green());

    // Cleanup old container
    let _ = Command::new("docker").args(["stop", "potkit-node"]).output();
    let _ = Command::new("docker").args(["rm", "potkit-node"]).output();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{spinner} {msg}")?);
    spinner.enable_steady_tick(Duration::from_millis(120));
    spinner.set_message("Starting Portaldot node...");

    let output = Command::new("docker")
        .args([
            "run",
            "-d",
            "-p", "9944:9944",
            "-p", "9933:9933",
            "--name", "potkit-node",
            "parity/substrate:latest",
            "--dev",
            "--rpc-external",
            "--rpc-cors", "all",
            "--rpc-methods", "unsafe",
        ])
        .output()?;

    if output.status.success() {
        spinner.finish_with_message("✅ Container started!");

        println!("\n{}", "⏳ Waiting 30 seconds for node to fully boot...".yellow());
        std::thread::sleep(Duration::from_secs(30));

        println!("\n{}", "📋 Recent Logs:".bold().cyan());
        let logs = Command::new("docker").args(["logs", "potkit-node", "--tail", "50"]).output()?;
        println!("{}", String::from_utf8_lossy(&logs.stdout));

        println!("\n{}", "📡 Node Information".bold().green());
        println!("RPC URL : ws://127.0.0.1:9944");
        println!("\nTry now:");
        println!("   cargo run -- chain-info");
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to start container:\n{}", err);
    }

    Ok(())
}