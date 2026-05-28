use anyhow::{Context, Result};
use colored::*;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub async fn run() -> Result<()> {
    println!();

    println!(
        "{}",
        "📘 PotKit ABI Inspection"
            .bold()
            .green()
    );

    println!();

    // ---------------------------------------------------
    // Locate Contract Metadata
    // ---------------------------------------------------

    let contract_path = Path::new("target")
        .join("ink")
        .join("real_contract.contract");

    if !contract_path.exists() {
        anyhow::bail!("Contract metadata not found");
    }

    println!(
        "{} {}",
        "✓ Contract metadata found:".green(),
        contract_path.display()
    );

    // ---------------------------------------------------
    // Read File
    // ---------------------------------------------------

    let contents = fs::read_to_string(&contract_path)
        .context("Failed to read .contract file")?;

    let json: Value = serde_json::from_str(&contents)
        .context("Failed to parse JSON metadata")?;

    println!();

    // ---------------------------------------------------
    // Contract Name
    // ---------------------------------------------------

    if let Some(name) = json
        .get("contract")
        .and_then(|c| c.get("name"))
        .and_then(|n| n.as_str())
    {
        println!(
            "{} {}",
            "Contract:".bright_cyan(),
            name.yellow()
        );
    }

    println!();

    // ---------------------------------------------------
    // Constructors
    // ---------------------------------------------------

    println!(
        "{}",
        "Constructors".bold().bright_green()
    );

    if let Some(constructors) = json
        .get("spec")
        .and_then(|s| s.get("constructors"))
        .and_then(|c| c.as_array())
    {
        for constructor in constructors {
            if let Some(label) = constructor.get("label") {
                if let Some(label_str) = label.as_str() {
                    println!(
                        "  • {}()",
                        label_str.yellow()
                    );
                } else if let Some(label_array) = label.as_array() {
                    let parts: Vec<String> = label_array
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect();

                    println!(
                        "  • {}()",
                        parts.join("::").yellow()
                    );
                }
            }
        }
    }

    println!();

    // ---------------------------------------------------
    // Messages
    // ---------------------------------------------------

    println!(
        "{}",
        "Messages".bold().bright_green()
    );

    if let Some(messages) = json
        .get("spec")
        .and_then(|s| s.get("messages"))
        .and_then(|m| m.as_array())
    {
        for message in messages {
            if let Some(label) = message.get("label") {
                if let Some(label_str) = label.as_str() {
                    println!(
                        "  • {}()",
                        label_str.cyan()
                    );
                } else if let Some(label_array) = label.as_array() {
                    let parts: Vec<String> = label_array
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect();

                    println!(
                        "  • {}()",
                        parts.join("::").cyan()
                    );
                }
            }
        }
    }

    println!();

    // ---------------------------------------------------
    // Events
    // ---------------------------------------------------

    println!(
        "{}",
        "Events".bold().bright_green()
    );

    if let Some(events) = json
        .get("spec")
        .and_then(|s| s.get("events"))
        .and_then(|e| e.as_array())
    {
        for event in events {
            if let Some(label) = event.get("label") {
                if let Some(label_str) = label.as_str() {
                    println!(
                        "  • {}",
                        label_str.magenta()
                    );
                } else if let Some(label_array) = label.as_array() {
                    let parts: Vec<String> = label_array
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect();

                    println!(
                        "  • {}",
                        parts.join("::").magenta()
                    );
                }
            }
        }
    }

    println!();

    println!(
        "{}",
        "🚀 ABI inspection completed".bold().green()
    );

    println!();

    Ok(())
}