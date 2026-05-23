use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;
use toml::Value;

use crate::cli::ConfigSetArgs;

fn config_path() -> Result<PathBuf> {
    let home = home_dir().context("Could not determine home directory")?;
    Ok(home.join(".potkit").join("config.toml"))
}

pub async fn init() -> Result<()> {
    let path = config_path()?;
    let parent = path.parent().unwrap();

    fs::create_dir_all(parent)?;

    let default_config = r#"
rpc_url = "ws://127.0.0.1:9944"
default_account = ""
network = "local"
"#;

    fs::write(&path, default_config.trim_start())?;

    println!("{}", "⚙️ PotKit configuration initialized.".bold().green());
    println!("Config file: {}", path.display().to_string().yellow());

    Ok(())
}

pub async fn set(args: ConfigSetArgs) -> Result<()> {
    let path = config_path()?;

    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Could not read {}", path.display()))?;

    let mut value: Value = toml::from_str(&contents)?;

    if let Some(table) = value.as_table_mut() {
        table.insert(args.key.clone(), Value::String(args.value.clone()));
    }

    fs::write(&path, toml::to_string_pretty(&value)?)?;

    println!(
        "{}",
        format!("✓ Updated '{}' successfully.", args.key)
            .bold()
            .green()
    );

    Ok(())
}

pub async fn show() -> Result<()> {
    let path = config_path()?;

    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Could not read {}", path.display()))?;

    println!("{}", "📄 Current PotKit Configuration".bold().cyan());
    println!();
    println!("{}", contents);

    Ok(())
}
