use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::Deserialize;

use super::{UsageProvider, UsageReport};

const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";

fn parse_api_keys(raw: &str) -> Vec<String> {
    raw.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[derive(Debug, Deserialize)]
struct Envelope<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct KeyStats {
    label: Option<String>,
    limit: Option<f64>,
    limit_reset: Option<String>,
    limit_remaining: Option<f64>,
    usage: f64,
    usage_daily: f64,
    usage_weekly: f64,
    usage_monthly: f64,
}

#[derive(Debug, Deserialize)]
struct CreditsStats {
    total_credits: f64,
    total_usage: f64,
}

pub struct OpenRouter {
    api_keys: Vec<String>,
}

impl OpenRouter {
    pub fn new(keys_str: &str) -> Self {
        Self {
            api_keys: parse_api_keys(keys_str),
        }
    }

    fn build_client(api_key: &str) -> Result<reqwest::Client> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {api_key}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value)
                .context("invalid API key for Authorization header")?,
        );

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")
    }

    async fn fetch_key_stats(client: &reqwest::Client) -> Result<KeyStats> {
        let response = client
            .get(format!("{OPENROUTER_BASE_URL}/key"))
            .send()
            .await
            .context("request to /key failed")?;

        let response = response
            .error_for_status()
            .context("OpenRouter /key returned an error status")?;

        let payload: Envelope<KeyStats> = response
            .json()
            .await
            .context("failed to parse /key JSON response")?;

        Ok(payload.data)
    }

    async fn fetch_credits_stats(client: &reqwest::Client) -> Result<CreditsStats> {
        let response = client
            .get(format!("{OPENROUTER_BASE_URL}/credits"))
            .send()
            .await
            .context("request to /credits failed")?;

        let response = response.error_for_status()?;
        let payload: Envelope<CreditsStats> = response.json().await?;
        Ok(payload.data)
    }
}

#[async_trait]
impl UsageProvider for OpenRouter {
    fn provider_name(&self) -> &'static str {
        "openrouter"
    }

    async fn check_usage(&self) -> Result<Vec<UsageReport>> {
        let mut reports = Vec::with_capacity(self.api_keys.len());

        for key in &self.api_keys {
            let client = Self::build_client(key)?;

            let key_stats = Self::fetch_key_stats(&client)
                .await
                .context("failed to fetch key details from /key")?;

            let credits_stats = Self::fetch_credits_stats(&client).await.ok();

            reports.push(UsageReport {
                provider: self.provider_name(),
                key_label: key_stats.label,
                limit: key_stats.limit,
                limit_remaining: key_stats.limit_remaining,
                limit_reset: key_stats.limit_reset,
                usage: key_stats.usage,
                usage_daily: key_stats.usage_daily,
                usage_weekly: key_stats.usage_weekly,
                usage_monthly: key_stats.usage_monthly,
                credits_total: credits_stats.as_ref().map(|c| c.total_credits),
                credits_used: credits_stats.as_ref().map(|c| c.total_usage),
            });
        }

        Ok(reports)
    }
}
