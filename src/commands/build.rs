use crate::utils::spinner::create_spinner;
use anyhow::{bail, Result};
use colored::*;
use std::path::Path;
use std::process::{Command, Stdio};

pub async fn run() -> Result<()> {
    println!("{}", "🛠️ PotKit Contract Build".bold().green());

    let detect_spinner = create_spinner("[1/3] Detecting cargo-contract...")?;
    let check = Command::new("cargo")
        .args(["contract", "--version"])
        .output();

    match check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            detect_spinner.finish_with_message("✓ cargo-contract available");
            println!("{} {}", "Version:".green(), version.trim().yellow());
        }
        _ => {
            bail!(
                "cargo-contract is not installed.\n\
                 Install it with:\n\
                 cargo install cargo-contract --locked"
            );
        }
    }

    let build_spinner = create_spinner("[2/3] Building ink! contract...")?;
    let status = Command::new("cargo")
        .args(["contract", "build"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Smart contract build failed.");
    }

    build_spinner.finish_with_message("✓ Contract compiled");

    let validate_spinner = create_spinner("[3/3] Validating artifacts...")?;
    let ink_dir = Path::new("target").join("ink");

    if ink_dir.exists() {
        validate_spinner.finish_with_message("✓ Wasm artifact ready");
    } else {
        bail!("Expected artifacts were not generated in target/ink");
    }

    println!();
    println!("{}", "🚀 Build completed successfully".bold().green());

    Ok(())
}
