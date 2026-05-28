use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub fn create_spinner(message: &str) -> Result<ProgressBar> {
    let spinner = ProgressBar::new_spinner();

    spinner.set_style(ProgressStyle::with_template("{spinner:.cyan} {msg}")?);

    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    spinner.set_message(message.to_string());

    Ok(spinner)
}
