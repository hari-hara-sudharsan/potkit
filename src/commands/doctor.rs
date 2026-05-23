use anyhow::Result;
use colored::*;
use dirs::home_dir;
use std::path::PathBuf;
use std::process::Command;

pub async fn run() -> Result<()> {
    println!("{}", "🩺 PotKit Doctor".bold().green());
    println!();

    check_tool("Rust", "rustc", &["--version"]);
    check_tool("Cargo", "cargo", &["--version"]);
    check_tool("Docker", "docker", &["--version"]);
    check_tool("Git", "git", &["--version"]);

    println!(
        "{} {} {}",
        "✓".green(),
        "OS".bold(),
        format!(": {} ({})", std::env::consts::OS, std::env::consts::ARCH)
            .yellow()
    );

    check_config();

    println!();
    println!(
        "{}",
        "🎉 Environment diagnostics completed.".bold().green()
    );

    Ok(())
}

fn check_tool(name: &str, command: &str, args: &[&str]) {
    let output = Command::new(command).args(args).output();

    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout);
            let version = version.trim();

            println!(
                "{} {} {}",
                "✓".green(),
                format!("{:<8}", name).bold(),
                format!(": {}", version).yellow()
            );
        }
        _ => {
            println!(
                "{} {} {}",
                "✗".red(),
                format!("{:<8}", name).bold(),
                ": Not found".red()
            );
        }
    }
}

fn check_config() {
    let path: Option<PathBuf> = home_dir().map(|home| {
        home.join(".potkit").join("config.toml")
    });

    match path {
        Some(config_path) if config_path.exists() => {
            println!(
                "{} {} {}",
                "✓".green(),
                format!("{:<8}", "Config").bold(),
                format!(": {}", config_path.display()).yellow()
            );
        }
        Some(config_path) => {
            println!(
                "{} {} {}",
                "!".yellow(),
                format!("{:<8}", "Config").bold(),
                format!(": Not found ({})", config_path.display()).yellow()
            );
        }
        None => {
            println!(
                "{} {} {}",
                "✗".red(),
                format!("{:<8}", "Config").bold(),
                ": Could not determine home directory".red()
            );
        }
    }
}