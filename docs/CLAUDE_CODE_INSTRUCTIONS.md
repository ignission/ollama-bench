# Claude Code Instructions for ollama-bench Development

## Project Initialization

### 1. Create New Rust Project
```bash
# Initialize the project
cargo new ollama-bench --bin
cd ollama-bench

# Add to Cargo.toml immediately
```

### 2. Initial Cargo.toml Configuration
```toml
[package]
name = "ollama-bench"
version = "0.1.0"
edition = "2021"
rust-version = "1.70.0"
authors = ["Your Name <your.email@example.com>"]
description = "⚡ Apache Bench-style Ollama LLM performance benchmarking"
readme = "README.md"
repository = "https://github.com/username/ollama-bench"
license = "MIT OR Apache-2.0"
keywords = ["ollama", "benchmark", "llm", "performance", "cli"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
clap = { version = "4.4", features = ["derive", "color"] }
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
crossterm = "0.27"
anyhow = "1.0"
chrono = { version = "0.4", features = ["serde"] }

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.release-small]
inherits = "release"
opt-level = "z"
```

## Development Phases

### Phase 1: Core Infrastructure (Priority 1)

#### Files to Create:
1. **src/main.rs** - Entry point and CLI setup
2. **src/cli.rs** - Command-line interface definitions
3. **src/types.rs** - Core data structures
4. **src/error.rs** - Error handling
5. **src/config.rs** - Default configurations

#### Implementation Order:
```rust
// 1. First implement basic CLI parsing in src/cli.rs
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "ollama-bench")]
#[command(about = "⚡ Apache Bench-style Ollama LLM performance benchmarking")]
#[command(version)]
pub struct Cli {
    /// Models to benchmark (positional arguments for simplicity)
    pub models: Vec<String>,
    
    /// Number of test iterations per model
    #[arg(short, long, default_value = "5")]
    pub iterations: u32,
    
    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,
    
    /// Quiet mode
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}
```

```rust
// 2. Define core types in src/types.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub model: String,
    pub prompt: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub tokens_per_second: f64,
    pub time_to_first_token_ms: u64,
    pub total_duration_ms: u64,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelSummary {
    pub model: String,
    pub total_tests: u32,
    pub success_rate: f64,
    pub avg_tokens_per_second: f64,
    pub min_tokens_per_second: f64,
    pub max_tokens_per_second: f64,
    pub avg_ttft_ms: f64,
}
```

### Phase 2: Ollama Integration (Priority 2)

#### Files to Create:
1. **src/ollama.rs** - Ollama API client
2. **src/benchmark.rs** - Core benchmarking logic
3. **src/output.rs** - Output formatting

#### Critical Implementation Details:

```rust
// src/ollama.rs - Implement Ollama API communication
use reqwest::Client;
use serde_json::json;

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn list_models(&self) -> anyhow::Result<Vec<String>> {
        // Implement /api/tags endpoint
        // Return list of available models
    }
    
    pub async fn generate(&self, model: &str, prompt: &str) -> anyhow::Result<BenchmarkResult> {
        // Implement /api/generate endpoint
        // Measure timing and extract metrics
        // Return structured result
    }
    
    pub async fn health_check(&self) -> anyhow::Result<bool> {
        // Check if Ollama is running
        // Return connection status
    }
}
```

### Phase 3: CLI Integration (Priority 3)

#### Files to Create:
1. **src/runner.rs** - Main benchmark execution
2. **src/progress.rs** - Progress indication
3. **src/validation.rs** - Input validation

#### Key Implementation Focus:

```rust
// src/runner.rs - Main execution logic
pub struct BenchmarkRunner {
    client: OllamaClient,
    config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub async fn run(&self, models: Vec<String>) -> anyhow::Result<Vec<ModelSummary>> {
        // 1. Validate models exist
        // 2. Run benchmarks with progress indication
        // 3. Collect and aggregate results
        // 4. Return summaries
    }
    
    async fn benchmark_model(&self, model: &str) -> anyhow::Result<Vec<BenchmarkResult>> {
        // Single model benchmarking logic
        // Include error handling and retries
    }
}
```

## Critical Implementation Guidelines

### 1. Error Handling Priority
```rust
// Implement user-friendly error messages from the start
pub enum BenchmarkError {
    #[error("❌ Ollama is not running\n💡 Start with: ollama serve")]
    OllamaNotRunning,
    
    #[error("❌ Model '{0}' not found\n💡 Install with: ollama pull {0}")]
    ModelNotFound(String),
    
    #[error("❌ Network timeout after {0}s\n💡 Try increasing --timeout")]
    NetworkTimeout(u64),
}
```

### 2. Progress Indication
```rust
// Use crossterm for cross-platform terminal control
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor,
};

pub fn show_progress(current: u32, total: u32, model: &str) {
    let percentage = (current * 100) / total;
    let bar_length = 30;
    let filled = (bar_length * current) / total;
    
    let bar = "█".repeat(filled as usize) + &"▒".repeat((bar_length - filled) as usize);
    
    print!("\rTesting {}... {} {}% ({}/{})", model, bar, percentage, current, total);
    std::io::stdout().flush().unwrap();
}
```

### 3. Output Formatting
```rust
// Table output using crossterm for colors
pub fn print_results_table(summaries: &[ModelSummary]) {
    println!("\n┌─────────────┬─────────────┬─────────────┬──────────────┐");
    println!("│ Model       │ Avg Speed   │ TTFT        │ Success      │");
    println!("├─────────────┼─────────────┼─────────────┼──────────────┤");
    
    for summary in summaries {
        println!(
            "│ {:11} │ {:9.1} tok/s │ {:9.0}ms │ {:10.1}% │",
            summary.model,
            summary.avg_tokens_per_second,
            summary.avg_ttft_ms,
            summary.success_rate * 100.0
        );
    }
    
    println!("└─────────────┴─────────────┴─────────────┴──────────────┘");
    
    // Print winner
    if let Some(winner) = find_fastest(summaries) {
        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("\n🏆 Winner: {}\n", winner.model)),
            ResetColor
        ).unwrap();
    }
}
```

## Testing Strategy Implementation

### 1. Unit Tests Structure
```rust
// In each module, add comprehensive tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parsing() {
        // Test various CLI argument combinations
    }
    
    #[tokio::test]
    async fn test_ollama_client() {
        // Mock Ollama responses for testing
    }
    
    #[test]
    fn test_result_aggregation() {
        // Test statistical calculations
    }
}
```

### 2. Integration Tests
```rust
// tests/integration_tests.rs
use assert_cmd::Command;

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("ollama-bench").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("ollama-bench").unwrap();
    cmd.arg("--version").assert().success();
}
```

## Build Configuration

### 1. GitHub Actions Workflow
Create `.github/workflows/ci.yml`:
```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo test --all-features
    - run: cargo build --release
```

### 2. Cross-Compilation Setup
```bash
# Add targets for cross-compilation
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
```

## Quality Assurance Checklist

### Before Each Commit:
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `cargo clippy` shows no warnings
- [ ] `cargo fmt` applied
- [ ] All error messages are user-friendly
- [ ] Progress indication works correctly
- [ ] Cross-platform compatibility verified

### Before Release:
- [ ] All integration tests pass
- [ ] Binary size < 15MB
- [ ] Startup time < 100ms
- [ ] Memory usage < 10MB base
- [ ] Documentation updated
- [ ] CHANGELOG.md updated

## Documentation Requirements

### 1. README.md Structure
```markdown
# ollama-bench

⚡ Apache Bench-style Ollama LLM performance benchmarking

## Quick Start
\`\`\`bash
# Install
cargo install ollama-bench

# Basic usage
ollama-bench llama2:7b

# Compare models
ollama-bench llama2:7b mistral:7b
\`\`\`

## Features
- Zero dependencies (except Ollama)
- 5-minute benchmark guarantee
- Cross-platform native performance
- Beautiful terminal output
```

### 2. Code Documentation
```rust
/// Runs a benchmark test for a specific model with the given prompt
/// 
/// # Arguments
/// * `model` - The name of the Ollama model to test
/// * `prompt` - The test prompt to send to the model
/// 
/// # Returns
/// * `Result<BenchmarkResult>` - Success result with timing data or error
/// 
/// # Example
/// ```
/// let result = client.generate("llama2:7b", "Hello world").await?;
/// println!("Speed: {:.1} tokens/sec", result.tokens_per_second);
/// ```
pub async fn generate(&self, model: &str, prompt: &str) -> anyhow::Result<BenchmarkResult> {
    // Implementation
}
```

## Success Criteria

### MVP Release (v0.1.0):
1. ✅ Basic single model benchmarking
2. ✅ Clean terminal table output
3. ✅ Cross-platform binaries
4. ✅ User-friendly error messages
5. ✅ <5 minute first-run experience

### Production Release (v1.0.0):
1. ✅ Multi-model comparison
2. ✅ JSON/CSV output options
3. ✅ Progress indicators
4. ✅ Comprehensive error handling
5. ✅ Full documentation
6. ✅ Package manager distribution

---

This instruction set provides Claude Code with everything needed to develop a production-ready ollama-bench tool that solves the identified market gap.