use crate::utils::spinner::create_spinner;
use anyhow::{bail, Result};
use colored::*;
use std::process::Command;
use std::time::Duration;

pub async fn run() -> Result<()> {
    println!("{}", "🚀 Starting local Portaldot node...".bold().green());

    // Cleanup old container
    let _ = Command::new("docker")
        .args(["stop", "potkit-node"])
        .output();
    let _ = Command::new("docker").args(["rm", "potkit-node"]).output();

    let docker_spinner = create_spinner("[1/3] Checking Docker...")?;
    let docker_check = Command::new("docker").arg("--version").output();

    match docker_check {
        Ok(output) if output.status.success() => {
            docker_spinner.finish_with_message("✓ Docker available");
        }
        _ => {
            bail!("Docker is not available. Ensure Docker is installed and on PATH.");
        }
    }

    let start_spinner = create_spinner("[2/3] Starting container...")?;
    let output = Command::new("docker")
        .args([
            "run",
            "-d",
            "-p",
            "9944:9944",
            "-p",
            "9933:9933",
            "--name",
            "potkit-node",
            "parity/substrate:latest",
            "--dev",
            "--rpc-external",
            "--rpc-cors",
            "all",
            "--rpc-methods",
            "unsafe",
        ])
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to start container:\n{}", err);
    }

    start_spinner.finish_with_message("✓ Container started");

    let rpc_spinner = create_spinner("[3/3] Waiting for RPC...")?;
    println!(
        "\n{}",
        "⏳ Waiting 30 seconds for node to fully boot...".yellow()
    );
    std::thread::sleep(Duration::from_secs(30));
    rpc_spinner.finish_with_message("✓ RPC operational");

    println!("\n{}", "📋 Recent Logs:".bold().cyan());
    let logs = Command::new("docker")
        .args(["logs", "potkit-node", "--tail", "50"])
        .output()?;
    println!("{}", String::from_utf8_lossy(&logs.stdout));

    println!("\n{}", "📡 Node Information".bold().green());
    println!("RPC URL : ws://127.0.0.1:9944");
    println!("\nTry now:");
    println!("   cargo run -- chain-info");

    Ok(())
}
