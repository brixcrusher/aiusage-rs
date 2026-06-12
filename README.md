# aiusage-rs

CLI tool to retrieve usage and credit statistics for OpenRouter API keys from the terminal.

## Usage

```sh
# via env var (single or ;-delimited multiple keys)
export OPENROUTER_API_KEY="sk-or-v1-xxx;sk-or-v1-yyy"
cargo run

# via flag
cargo run -- --openrouter-key "sk-or-v1-xxx;sk-or-v1-yyy"
```

The tool queries `/key` (per-key limits and usage) and `/credits` (account-level balance) for each key and prints a summary report.

## Multiple API keys

Separate multiple keys with `;` in either `OPENROUTER_API_KEY` or `--openrouter-key`. Each key is checked independently.

## Future providers

The project is structured around a `UsageProvider` trait in `src/provider/`, making it straightforward to add OpenAI, Anthropic, or other providers.
