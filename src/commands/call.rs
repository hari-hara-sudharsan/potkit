use anyhow::{Context, Result};
use colored::*;
use dirs::home_dir;
use std::fs;

use subxt::{OnlineClient, PolkadotConfig};
use subxt::dynamic::Value;
use subxt_signer::sr25519::dev;

pub async fn run(message: String) -> Result<()> {

    println!();

    println!(
        "{}",
        "📦 PotKit Contract Call"
            .bold()
            .green()
    );

    println!();

    // ---------------------------------------------------
    // Validate message
    // ---------------------------------------------------

    if message != "flip" {
        anyhow::bail!("Only 'flip' supported currently");
    }

    // ---------------------------------------------------
    // Load config
    // ---------------------------------------------------

    let config_path = home_dir()
        .context("Could not determine home directory")?
        .join(".potkit")
        .join("config.toml");

    let config_contents =
        fs::read_to_string(&config_path)?;

    let value: toml::Value =
        toml::from_str(&config_contents)?;

    let rpc_url = value
        .get("rpc_url")
        .and_then(|v| v.as_str())
        .unwrap_or("ws://127.0.0.1:9944");

    println!(
        "{} {}",
        "RPC URL:".bright_cyan(),
        rpc_url.yellow()
    );

    // ---------------------------------------------------
    // Connect
    // ---------------------------------------------------

    println!(
        "{}",
        "🔌 Connecting to blockchain..."
            .cyan()
    );

    let api =
        OnlineClient::<PolkadotConfig>
            ::from_url(rpc_url)
            .await?;

    println!(
        "{}",
        "✓ Connected".green()
    );

    // ---------------------------------------------------
    // Contract details
    // ---------------------------------------------------

    let contract_address =
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstf9dKz6P5Qx8hV2o1Y";

    println!();

    println!(
        "{} {}",
        "Contract:".bright_cyan(),
        contract_address.yellow()
    );

    println!(
        "{} {}()",
        "Message:".bright_cyan(),
        message.yellow()
    );

    // ---------------------------------------------------
    // Signer
    // ---------------------------------------------------

    let signer = dev::alice();

    // ---------------------------------------------------
    // flip() selector
    // ---------------------------------------------------

    let selector: [u8; 4] =
        [0x63, 0x3a, 0xa5, 0x51];

    println!();

    println!(
        "{} {:?}",
        "Selector:".bright_blue(),
        selector
    );

    // ---------------------------------------------------
    // Encode selector
    // ---------------------------------------------------

    let mut data = Vec::new();

    data.extend_from_slice(&selector);

    println!();

    println!(
        "{}",
        "⏳ Submitting contract transaction..."
            .cyan()
    );

    // ---------------------------------------------------
    // Build Dynamic Extrinsic
    // ---------------------------------------------------

    let tx = subxt::dynamic::tx(
        "Contracts",
        "call",
        vec![

            // dest: MultiAddress::Id(AccountId32)
            Value::variant(
                "Id",
                vec![
                    Value::from_bytes(
                        hex::decode(
                            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                        )?
                    )
                ].into()
            ),

            // value
            Value::u128(0),

            // gas_limit: WeightV2
            Value::named_composite(vec![

                (
                    "ref_time",
                    Value::u128(500_000_000_000)
                ),

                (
                    "proof_size",
                    Value::u128(0)
                ),

            ]),

            // storage_deposit_limit = None
            Value::unnamed_variant(
                "None",
                vec![]
            ),

            // input data
            Value::from_bytes(&data),
        ],
    );

    // ---------------------------------------------------
    // Submit Transaction
    // ---------------------------------------------------

    let tx_progress = api
        .tx()
        .sign_and_submit_then_watch_default(
            &tx,
            &signer
        )
        .await?;

    let _events = tx_progress
        .wait_for_finalized_success()
        .await?;

    // ---------------------------------------------------
    // Print Result
    // ---------------------------------------------------

    println!();

    println!(
        "{}",
        "✓ Contract call finalized"
            .green()
    );

    println!(
        "{}",
        "Transaction executed successfully"
            .bright_green()
    );

    println!();

    Ok(())
}