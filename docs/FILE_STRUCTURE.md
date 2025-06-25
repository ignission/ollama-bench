# ollama-meter Project File Structure

## Complete Directory Layout

```
ollama-meter/
├── Cargo.toml                 # Project configuration and dependencies
├── Cargo.lock                 # Dependency lock file (auto-generated)
├── README.md                  # Project documentation
├── LICENSE                    # MIT or Apache-2.0 license
├── CHANGELOG.md               # Version history and changes
├── .gitignore                 # Git ignore rules
├── .github/
│   └── workflows/
│       ├── ci.yml             # Continuous integration
│       ├── release.yml        # Release automation
│       └── security.yml       # Security scanning
├── src/
│   ├── main.rs                # Application entry point
│   ├── lib.rs                 # Library root (for unit testing)
│   ├── cli.rs                 # Command-line interface definitions
│   ├── types.rs               # Core data structures and types
│   ├── error.rs               # Error handling and user-friendly messages
│   ├── config.rs              # Default configurations and constants
│   ├── ollama.rs              # Ollama API client implementation
│   ├── benchmark.rs           # Core benchmarking logic
│   ├── runner.rs              # Main benchmark execution orchestration
│   ├── output.rs              # Output formatting (table, JSON, CSV)
│   ├── progress.rs            # Progress indication and user feedback
│   ├── validation.rs          # Input validation and sanitization
│   ├── utils.rs               # Utility functions and helpers
│   └── html.rs                # HTML report generation (Phase 3)
├── tests/
│   ├── integration_tests.rs   # End-to-end integration tests
│   ├── cli_tests.rs           # CLI interface testing
│   └── fixtures/
│       ├── mock_responses/    # Mock Ollama API responses
│       └── test_configs/      # Test configuration files
├── docs/
│   ├── CONTRIBUTING.md        # Contribution guidelines
│   ├── DEVELOPMENT.md         # Development setup instructions
│   ├── API.md                 # Ollama API interaction details
│   └── PERFORMANCE.md         # Performance benchmarks and targets
├── examples/
│   ├── basic_usage.sh         # Basic usage examples
│   ├── advanced_usage.sh      # Advanced configuration examples
│   └── config_examples/
│       ├── simple.toml        # Simple configuration file
│       └── comprehensive.toml # Full-featured configuration
├── scripts/
│   ├── build_release.sh       # Cross-platform build script
│   ├── install.sh             # Installation script
│   └── benchmark_self.sh      # Self-benchmarking script
└── assets/
    ├── logo.png               # Project logo
    ├── screenshot.png         # Terminal output screenshot
    └── html_template.html     # HTML report template
```

## File Implementation Priority

### Phase 1: Core Infrastructure (Essential)

#### 1. Cargo.toml
```toml
[package]
name = "ollama-meter"
version = "0.1.0"
edition = "2021"
rust-version = "1.70.0"
authors = ["Your Name <your.email@example.com>"]
description = "⚡ Apache Bench-style Ollama LLM performance benchmarking"
readme = "README.md"
repository = "https://github.com/username/ollama-meter"
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

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"
tokio-test = "0.4"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[[bin]]
name = "ollama-meter"
path = "src/main.rs"
```

#### 2. src/main.rs
```rust
//! ollama-meter: Apache Bench-style Ollama LLM performance benchmarking
//! 
//! This tool provides simple, fast performance benchmarking for Ollama models
//! with a focus on ease of use and cross-platform compatibility.

use clap::Parser;
use anyhow::Result;

mod cli;
mod types;
mod error;
mod config;
mod ollama;
mod benchmark;
mod runner;
mod output;
mod progress;
mod validation;
mod utils;

use cli::Cli;
use runner::BenchmarkRunner;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize runner with configuration
    let runner = BenchmarkRunner::new(cli).await?;
    
    // Execute benchmarks
    let results = runner.run().await?;
    
    // Output results
    runner.output_results(results).await?;
    
    Ok(())
}
```

#### 3. src/cli.rs
```rust
//! Command-line interface definitions and parsing

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ollama-meter")]
#[command(about = "⚡ Apache Bench-style Ollama LLM performance benchmarking")]
#[command(version)]
#[command(long_about = None)]
pub struct Cli {
    /// Models to benchmark (space separated for simplicity)
    /// Example: ollama-meter llama2:7b mistral:7b
    pub models: Vec<String>,
    
    /// Number of test iterations per model
    #[arg(short = 'n', long, default_value = "5")]
    pub iterations: u32,
    
    /// Concurrent requests (experimental)
    #[arg(short, long, default_value = "1")]
    pub concurrency: u32,
    
    /// Custom test prompts (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    pub prompts: Option<Vec<String>>,
    
    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,
    
    /// Output file path
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    
    /// Generate HTML report
    #[arg(long)]
    pub html: Option<PathBuf>,
    
    /// Ollama server URL
    #[arg(long, default_value = "http://localhost:11434")]
    pub url: String,
    
    /// Timeout per request (seconds)
    #[arg(long, default_value = "30")]
    pub timeout: u64,
    
    /// Quiet mode (minimal output)
    #[arg(short, long)]
    pub quiet: bool,
    
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Skip warmup iteration
    #[arg(long)]
    pub no_warmup: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

impl Cli {
    /// Validate CLI arguments and provide helpful error messages
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.models.is_empty() {
            return Err(crate::error::BenchmarkError::NoModelsSpecified);
        }
        
        if self.iterations == 0 {
            return Err(crate::error::BenchmarkError::InvalidIterations);
        }
        
        Ok(())
    }
}
```

#### 4. src/types.rs
```rust
//! Core data structures and type definitions

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::time::Duration;

/// Result of a single benchmark test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Model name that was tested
    pub model: String,
    /// Test prompt used
    pub prompt: String,
    /// When the test was executed
    pub timestamp: DateTime<Utc>,
    /// Whether the test completed successfully
    pub success: bool,
    /// Tokens generated per second
    pub tokens_per_second: f64,
    /// Time to receive first token (milliseconds)
    pub time_to_first_token_ms: u64,
    /// Total duration of the request (milliseconds)
    pub total_duration_ms: u64,
    /// Number of prompt tokens processed
    pub prompt_tokens: u32,
    /// Number of completion tokens generated
    pub completion_tokens: u32,
    /// Error message if test failed
    pub error: Option<String>,
}

/// Aggregated statistics for a model across multiple tests
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelSummary {
    /// Model name
    pub model: String,
    /// Total number of tests run
    pub total_tests: u32,
    /// Percentage of successful tests
    pub success_rate: f64,
    /// Average tokens per second across all successful tests
    pub avg_tokens_per_second: f64,
    /// Minimum tokens per second observed
    pub min_tokens_per_second: f64,
    /// Maximum tokens per second observed
    pub max_tokens_per_second: f64,
    /// Average time to first token (milliseconds)
    pub avg_ttft_ms: f64,
    /// Fastest prompt (by tokens/sec)
    pub fastest_prompt: Option<String>,
    /// Slowest prompt (by tokens/sec)
    pub slowest_prompt: Option<String>,
}

/// Complete benchmark report containing all results and metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkReport {
    /// Report metadata
    pub metadata: ReportMetadata,
    /// Individual test results
    pub results: Vec<BenchmarkResult>,
    /// Per-model summary statistics
    pub summaries: Vec<ModelSummary>,
    /// Overall winner (fastest model)
    pub winner: Option<ModelSummary>,
}

/// Metadata about the benchmark run
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Tool version
    pub version: String,
    /// When the benchmark was run
    pub timestamp: DateTime<Utc>,
    /// Total duration of benchmark run
    pub total_duration_seconds: u64,
    /// Configuration used
    pub config: BenchmarkConfig,
}

/// Configuration for benchmark execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Number of iterations per model
    pub iterations: u32,
    /// Concurrent requests
    pub concurrency: u32,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Test prompts used
    pub prompts: Vec<String>,
    /// Whether warmup iteration was performed
    pub warmup_enabled: bool,
    /// Ollama server URL
    pub server_url: String,
}

/// Ollama API response for model generation
#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u64>,
}

/// Request payload for Ollama API
#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub options: Option<OllamaOptions>,
}

/// Optional parameters for Ollama requests
#[derive(Debug, Serialize)]
pub struct OllamaOptions {
    pub temperature: f32,
    pub top_p: f32,
    pub num_predict: i32,
}

impl Default for OllamaOptions {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            top_p: 0.9,
            num_predict: 100,
        }
    }
}
```

#### 5. src/error.rs
```rust
//! User-friendly error handling and messages

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BenchmarkError>;

#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("❌ Ollama is not running\n💡 Start with: ollama serve\n💡 Or check if it's running on a different port")]
    OllamaNotRunning,
    
    #[error("❌ Model '{0}' not found\n💡 Install with: ollama pull {0}\n💡 List available models: ollama list")]
    ModelNotFound(String),
    
    #[error("❌ Network timeout after {0}s\n💡 Try increasing timeout with --timeout {1}\n💡 Check if Ollama server is responsive")]
    NetworkTimeout(u64, u64),
    
    #[error("❌ No models specified\n💡 Usage: ollama-meter llama2:7b\n💡 Or: ollama-meter llama2:7b mistral:7b")]
    NoModelsSpecified,
    
    #[error("❌ Invalid number of iterations (must be > 0)\n💡 Use: --iterations 5")]
    InvalidIterations,
    
    #[error("❌ Cannot write to file: {0}\n💡 Check file permissions and disk space")]
    FileWriteError(String),
    
    #[error("❌ Invalid URL: {0}\n💡 Use format: http://localhost:11434\n💡 Or: http://your-server:11434")]
    InvalidUrl(String),
    
    #[error("❌ Insufficient memory for model '{0}'\n💡 Try a smaller model or free up system memory")]
    InsufficientMemory(String),
    
    #[error("❌ Request failed: {0}\n💡 Check Ollama logs: ollama logs")]
    RequestFailed(String),
    
    #[error("❌ Unexpected response format from Ollama\n💡 Check Ollama version compatibility")]
    InvalidResponse,
    
    #[error("❌ JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("❌ HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("❌ IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl BenchmarkError {
    /// Get a suggested fix for the error
    pub fn suggestion(&self) -> &str {
        match self {
            BenchmarkError::OllamaNotRunning => "Start Ollama with 'ollama serve'",
            BenchmarkError::ModelNotFound(_) => "Install the model with 'ollama pull <model>'",
            BenchmarkError::NetworkTimeout(_, _) => "Increase timeout or check server responsiveness",
            BenchmarkError::NoModelsSpecified => "Specify at least one model to benchmark",
            BenchmarkError::InvalidIterations => "Use a positive number for iterations",
            BenchmarkError::FileWriteError(_) => "Check file permissions and available disk space",
            BenchmarkError::InvalidUrl(_) => "Use a valid HTTP URL format",
            BenchmarkError::InsufficientMemory(_) => "Try a smaller model or free up memory",
            BenchmarkError::RequestFailed(_) => "Check Ollama server logs for details",
            BenchmarkError::InvalidResponse => "Update Ollama to the latest version",
            _ => "Check the error message for details",
        }
    }
    
    /// Check if the error is recoverable (can continue with other models)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            BenchmarkError::ModelNotFound(_)
                | BenchmarkError::InsufficientMemory(_)
                | BenchmarkError::RequestFailed(_)
                | BenchmarkError::NetworkTimeout(_, _)
        )
    }
}
```

#### 6. src/config.rs
```rust
//! Default configurations and constants

/// Default test prompts that provide good benchmarking coverage
pub const DEFAULT_PROMPTS: &[&str] = &[
    "Hello, how are you?",
    "Explain artificial intelligence in simple terms.",
    "Write a short Python function to calculate fibonacci numbers.",
    "What are the benefits of renewable energy?",
];

/// Minimum response length to ensure meaningful timing
pub const MIN_RESPONSE_TOKENS: u32 = 10;

/// Maximum response length to prevent runaway generations
pub const MAX_RESPONSE_TOKENS: u32 = 150;

/// Default Ollama server URL
pub const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";

/// Default number of benchmark iterations
pub const DEFAULT_ITERATIONS: u32 = 5;

/// Default request timeout in seconds
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Benchmark configuration with sensible defaults
#[derive(Debug, Clone)]
pub struct DefaultConfig;

impl DefaultConfig {
    /// Get default test prompts
    pub fn prompts() -> Vec<String> {
        DEFAULT_PROMPTS.iter().map(|s| s.to_string()).collect()
    }
    
    /// Get default Ollama request options
    pub fn ollama_options() -> crate::types::OllamaOptions {
        crate::types::OllamaOptions {
            temperature: 0.7,
            top_p: 0.9,
            num_predict: MAX_RESPONSE_TOKENS as i32,
        }
    }
}
```

### Phase 2: Core Functionality (High Priority)

#### 7. src/ollama.rs - Ollama API Client
#### 8. src/benchmark.rs - Benchmarking Logic  
#### 9. src/runner.rs - Execution Orchestration
#### 10. src/output.rs - Result Formatting
#### 11. src/progress.rs - Progress Indication
#### 12. src/validation.rs - Input Validation

### Phase 3: Advanced Features (Medium Priority)

#### 13. src/html.rs - HTML Report Generation
#### 14. src/utils.rs - Utility Functions
#### 15. tests/ - Comprehensive Test Suite
#### 16. docs/ - Documentation
#### 17. examples/ - Usage Examples

### Phase 4: Distribution (Lower Priority)

#### 18. .github/workflows/ - CI/CD Pipeline
#### 19. scripts/ - Build and Installation Scripts
#### 20. assets/ - Project Assets

## Implementation Guidelines

### File Dependency Order
1. **types.rs** → **error.rs** → **config.rs** (Foundation)
2. **cli.rs** → **validation.rs** (Interface)
3. **ollama.rs** → **benchmark.rs** (Core Logic)
4. **progress.rs** → **output.rs** (User Experience)
5. **runner.rs** → **main.rs** (Integration)

### Code Organization Principles
- **Single Responsibility**: Each module has one clear purpose
- **Dependency Injection**: Pass configuration and clients as parameters
- **Error Propagation**: Use `?` operator and structured error types
- **Testing**: Every public function should have unit tests
- **Documentation**: All public APIs must have rustdoc comments

### Quality Standards
- **Clippy**: No warnings in release builds
- **Formatting**: Use `cargo fmt` consistently
- **Testing**: Minimum 80% code coverage
- **Performance**: Binary size < 15MB, startup < 100ms
- **Cross-platform**: Test on Windows, macOS, and Linux

This file structure provides a solid foundation for building a professional-grade CLI tool that meets all the requirements identified in the market analysis.