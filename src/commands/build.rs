use anyhow::{bail, Result};
use colored::*;
use std::process::{Command, Stdio};

pub async fn run() -> Result<()> {
    println!("{}", "🛠️ Building smart contract...".bold().green());

    // Check whether cargo-contract is installed
    let check = Command::new("cargo")
        .args(["contract", "--version"])
        .output();

    match check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("{} {}", "✓ cargo-contract detected:".green(), version.trim());
        }
        _ => {
            bail!(
                "cargo-contract is not installed.\n\
                 Install it with:\n\
                 cargo install cargo-contract --locked"
            );
        }
    }

    println!("{}", "⏳ Running `cargo contract build`...".cyan());
    println!();

    // Execute cargo contract build and stream output directly
    let status = Command::new("cargo")
        .args(["contract", "build"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Smart contract build failed.");
    }

    println!();
    println!("{}", "✓ Contract built successfully.".bold().green());
    println!("{}", "📦 Check the `target/ink/` directory for artifacts.".yellow());

    Ok(())
}
