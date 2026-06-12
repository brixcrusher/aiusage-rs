use anyhow::{Context, Result};
use clap::Parser;
use std::env;

mod provider;
use provider::{OpenRouter, UsageProvider, UsageReport};

#[derive(Parser, Debug)]
#[command(
    name = "aiusage-rs",
    about = "Show AI provider key limits and credit usage statistics"
)]
struct Cli {
    /// OpenRouter API key(s), for example: --openrouter-key=sk-or-v1-... (use ; as delimiter for multiple keys)
    #[arg(long)]
    openrouter_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let raw_key = resolve_api_key(cli.openrouter_key)?;
    let provider = OpenRouter::new(&raw_key);
    let reports = provider.check_usage().await?;

    for report in &reports {
        print_report(report);
    }

    Ok(())
}

fn resolve_api_key(key_arg: Option<String>) -> Result<String> {
    if let Some(key) = key_arg {
        return Ok(key);
    }

    env::var("OPENROUTER_API_KEY").context(
        "no API key provided; pass --openrouter-key or set OPENROUTER_API_KEY in your environment",
    )
}

fn print_report(report: &UsageReport) {
    println!("Provider:{} usage report", report.provider);
    println!("==========================");

    println!(
        "Key label: {}",
        report.key_label.as_deref().unwrap_or("not provided by API")
    );
    println!("Limit (USD): {}", display_optional_money(report.limit));
    println!(
        "Limit remaining (USD): {}",
        display_optional_money(report.limit_remaining)
    );
    println!(
        "Limit reset policy: {}",
        report.limit_reset.as_deref().unwrap_or("none")
    );
    println!();

    println!("Usage (USD)");
    println!("- Total used: ${:.4}", report.usage);
    println!("- Daily used: ${:.4}", report.usage_daily);
    println!("- Weekly used: ${:.4}", report.usage_weekly);
    println!("- Monthly used: ${:.4}", report.usage_monthly);
    println!();

    match report.credits_total {
        Some(total) => {
            let credits_used = report.credits_used.unwrap_or(0.0);
            let remaining = total - credits_used;
            println!("Account credits (from /credits)");
            println!("- Total credits purchased: ${:.4}", total);
            println!("- Total credits used: ${:.4}", credits_used);
            println!("- Remaining account credits: ${:.4}", remaining);
        }
        None => {
            println!("Account credits (from /credits)");
            println!("- Could not read /credits with this key (management key may be required).");
        }
    }
    println!();
}

fn display_optional_money(value: Option<f64>) -> String {
    match value {
        Some(v) => format!("${v:.4}"),
        None => "unlimited".to_string(),
    }
}
