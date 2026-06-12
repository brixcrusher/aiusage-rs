mod openrouter;
pub use openrouter::OpenRouter;

use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct UsageReport {
    pub provider: &'static str,
    pub key_label: Option<String>,
    pub limit: Option<f64>,
    pub limit_remaining: Option<f64>,
    pub limit_reset: Option<String>,
    pub usage: f64,
    pub usage_daily: f64,
    pub usage_weekly: f64,
    pub usage_monthly: f64,
    pub credits_total: Option<f64>,
    pub credits_used: Option<f64>,
}

#[async_trait]
pub trait UsageProvider: Send + Sync {
    fn provider_name(&self) -> &'static str;
    async fn check_usage(&self) -> Result<Vec<UsageReport>>;
}
