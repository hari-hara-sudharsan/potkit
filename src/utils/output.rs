use chrono::Local;
use colored::*;

fn timestamp() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn chain_event(message: &str) {
    println!(
        "{} {} {}",
        format!("[{}]", timestamp()).bright_black(),
        "⛓".bright_blue(),
        message.green()
    );
}

pub fn tx_event(message: &str) {
    println!(
        "{} {} {}",
        format!("[{}]", timestamp()).bright_black(),
        "💧".cyan(),
        message.yellow()
    );
}

pub fn upload_event(message: &str) {
    println!(
        "{} {} {}",
        format!("[{}]", timestamp()).bright_black(),
        "📤".magenta(),
        message.green()
    );
}

pub fn deploy_event(message: &str) {
    println!(
        "{} {} {}",
        format!("[{}]", timestamp()).bright_black(),
        "📦".bright_cyan(),
        message.green()
    );
}

pub fn watch_event(message: &str) {
    println!(
        "{} {} {}",
        format!("[{}]", timestamp()).bright_black(),
        "👀".yellow(),
        message.white()
    );
}

pub fn build_event(message: &str) {
    println!(
        "{} {} {}",
        format!("[{}]", timestamp()).bright_black(),
        "🛠".bright_green(),
        message.green()
    );
}

pub fn print_chain_event(message: &str) {
    chain_event(message);
}
