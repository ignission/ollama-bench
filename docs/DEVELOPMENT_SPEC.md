# ollama-bench: Development Specification

## Project Overview

**ollama-bench** is a Rust-based CLI tool designed to provide Apache Bench-style simplicity for Ollama LLM performance benchmarking. It solves the "5-minute benchmark" problem that existing overengineered tools fail to address.

## Core Design Principles

### 1. "Just Works" Experience
- Zero dependencies beyond Ollama
- Single binary distribution
- 5-minute guarantee from download to results
- Cross-platform native performance

### 2. Apache Bench-Inspired Simplicity
```bash
# Simple as ab command
ollama-bench -m llama2:7b,mistral:7b -n 10
```

### 3. Progressive Complexity
- Level 1: `ollama-bench llama2:7b` (ultra-simple)
- Level 2: `ollama-bench llama2:7b mistral:7b` (comparison)
- Level 3: Full options with JSON/HTML output

## Technical Requirements

### Core Technology Stack
- **Language**: Rust 2021 Edition
- **Minimum Rust Version**: 1.70.0
- **Target Platforms**: 
  - Windows (x86_64-pc-windows-msvc)
  - macOS Intel (x86_64-apple-darwin)
  - macOS Apple Silicon (aarch64-apple-darwin)
  - Linux (x86_64-unknown-linux-gnu)

### Essential Dependencies
```toml
[dependencies]
clap = { version = "4.4", features = ["derive", "color"] }
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
crossterm = "0.27"
anyhow = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### Performance Targets
- **Binary size**: <15MB (release build)
- **Startup time**: <100ms
- **Memory usage**: <10MB base + model overhead
- **Benchmark completion**: 3-5 minutes for standard tests

## Command Line Interface Design

### Basic Usage Patterns
```bash
# Simplest usage
ollama-bench llama2:7b

# Model comparison
ollama-bench llama2:7b mistral:7b

# With options
ollama-bench -m llama2:7b,mistral:7b -n 10 -c 3

# Full featured
ollama-bench --models llama2:7b,mistral:7b \
             --iterations 10 \
             --concurrency 3 \
             --output json \
             --file results.json \
             --html report.html
```

### CLI Structure
```rust
#[derive(Parser)]
#[command(name = "ollama-bench")]
#[command(about = "⚡ Apache Bench-style Ollama LLM performance benchmarking")]
struct Cli {
    /// Models to benchmark (space or comma separated)
    #[arg(short, long, value_delimiter = ',')]
    models: Option<Vec<String>>,
    
    /// Number of test iterations per model
    #[arg(short, long, default_value = "5")]
    iterations: u32,
    
    /// Concurrent requests
    #[arg(short, long, default_value = "1")]
    concurrency: u32,
    
    /// Test prompts
    #[arg(short, long, value_delimiter = ',')]
    prompts: Option<Vec<String>>,
    
    /// Output format (table, json, csv)
    #[arg(short, long, default_value = "table")]
    output: OutputFormat,
    
    /// Output file path
    #[arg(short, long)]
    file: Option<PathBuf>,
    
    /// Generate HTML report
    #[arg(long)]
    html: Option<PathBuf>,
    
    /// Ollama server URL
    #[arg(long, default_value = "http://localhost:11434")]
    url: String,
    
    /// Timeout per request (seconds)
    #[arg(long, default_value = "30")]
    timeout: u64,
    
    /// Quiet mode (minimal output)
    #[arg(short, long)]
    quiet: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Models as positional arguments (for simplicity)
    models_pos: Vec<String>,
}
```

## Core Data Structures

### Benchmark Results
```rust
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
    pub memory_usage_mb: Option<u64>,
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
    pub avg_memory_mb: Option<f64>,
    pub fastest_prompt: Option<String>,
    pub slowest_prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub metadata: ReportMetadata,
    pub results: Vec<BenchmarkResult>,
    pub summaries: Vec<ModelSummary>,
    pub winner: Option<ModelSummary>,
}
```

## Feature Implementation Priority

### Phase 1: MVP (Essential Features)
1. **Basic benchmarking** - Single model, simple prompts
2. **Model comparison** - Multiple models side-by-side
3. **Table output** - Clean, colored terminal output
4. **Error handling** - Helpful error messages
5. **Cross-platform builds** - Windows, macOS, Linux

### Phase 2: Enhanced Features
1. **JSON/CSV output** - Machine-readable formats
2. **Custom prompts** - User-defined test prompts
3. **Concurrency support** - Parallel request testing
4. **Progress indicators** - Real-time progress display
5. **Configuration files** - Preset configurations

### Phase 3: Advanced Features
1. **HTML reports** - Beautiful visual reports
2. **System monitoring** - CPU/Memory usage tracking
3. **Historical comparison** - Compare with previous runs
4. **Auto-update** - Self-updating mechanism
5. **Prompt templates** - Predefined prompt sets

## Default Test Configuration

### Standard Prompts
```rust
pub const DEFAULT_PROMPTS: &[&str] = &[
    "Hello, how are you?",
    "Explain artificial intelligence in simple terms.",
    "Write a short Python function to calculate fibonacci numbers.",
    "What are the benefits of renewable energy?",
];
```

### Benchmark Parameters
```rust
pub struct BenchmarkConfig {
    pub iterations: u32,           // Default: 5
    pub concurrency: u32,          // Default: 1
    pub timeout_seconds: u64,      // Default: 30
    pub warmup_iterations: u32,    // Default: 1
    pub min_response_tokens: u32,  // Default: 10
    pub max_response_tokens: u32,  // Default: 100
}
```

## Output Formats

### 1. Terminal Table (Default)
```
⚡ ollama-bench v1.0.0 - Benchmarking 2 models with 5 iterations each

Testing llama2:7b...
████████████████████████████████ 100% (5/5) [00:02:15]

Testing mistral:7b...
████████████████████████████████ 100% (5/5) [00:01:58]

┌─────────────┬─────────────┬─────────────┬──────────────┬─────────────┐
│ Model       │ Avg Speed   │ TTFT        │ Memory      │ Success     │
├─────────────┼─────────────┼─────────────┼──────────────┼─────────────┤
│ llama2:7b   │ 28.5 tok/s  │ 234ms       │ 4.2 GB      │ 100%        │
│ mistral:7b  │ 31.2 tok/s  │ 198ms       │ 4.1 GB      │ 100%        │
└─────────────┴─────────────┴─────────────┴──────────────┴─────────────┘

🏆 Winner: mistral:7b (9.5% faster, 15% lower TTFT)
📊 Completed in 4m 13s
```

### 2. JSON Output
```json
{
  "metadata": {
    "timestamp": "2025-06-25T14:30:22Z",
    "version": "1.0.0",
    "total_duration_seconds": 253,
    "config": {
      "iterations": 5,
      "concurrency": 1,
      "prompts": ["Hello, how are you?", "..."]
    }
  },
  "results": [...],
  "summaries": [...],
  "winner": {
    "model": "mistral:7b",
    "avg_tokens_per_second": 31.2,
    "advantage_percent": 9.5
  }
}
```

### 3. HTML Report (Optional)
- Modern responsive design
- Interactive charts (Chart.js)
- Detailed comparison tables
- Exportable/shareable format

## Error Handling Strategy

### User-Friendly Error Messages
```rust
pub enum BenchmarkError {
    OllamaNotRunning,
    ModelNotFound(String),
    NetworkTimeout,
    InvalidResponse,
    InsufficientMemory,
}

impl Display for BenchmarkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchmarkError::OllamaNotRunning => {
                write!(f, "❌ Ollama is not running\n💡 Start with: ollama serve")
            }
            BenchmarkError::ModelNotFound(model) => {
                write!(f, "❌ Model '{}' not found\n💡 Install with: ollama pull {}", model, model)
            }
            // ... other error types
        }
    }
}
```

### Graceful Degradation
- Continue testing other models if one fails
- Provide partial results when possible
- Clear indication of failed tests in output
- Suggestions for resolving common issues

## Testing Strategy

### Unit Tests
- CLI argument parsing
- JSON serialization/deserialization
- Error handling scenarios
- Mathematical calculations (averages, percentiles)

### Integration Tests
- Mock Ollama server responses
- End-to-end command execution
- Output format validation
- Cross-platform compatibility

### Performance Tests
- Binary size verification
- Startup time measurement
- Memory usage monitoring
- Benchmark completion time

## Build and Release Process

### Cargo Configuration
```toml
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

### Cross-Compilation Targets
```bash
# Build for all platforms
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
```

### GitHub Actions Workflow
- Automated testing on all platforms
- Release builds for each target
- Automatic binary uploads to GitHub Releases
- Homebrew formula updates
- Cargo.io publishing

## Documentation Requirements

### README.md
- Clear installation instructions
- Quick start examples
- Feature overview
- Comparison with existing tools

### CLI Help
- Comprehensive `--help` output
- Examples for common use cases
- Error message guidance

### Man Page
- Traditional Unix documentation
- Installation via package managers

## Security Considerations

### Input Validation
- Sanitize user-provided prompts
- Validate model names
- URL validation for Ollama server
- File path validation for outputs

### Network Security
- HTTPS support for remote Ollama instances
- Request timeout enforcement
- Rate limiting considerations

## Monitoring and Analytics

### Optional Telemetry
- Anonymous usage statistics (opt-in)
- Performance metrics collection
- Error reporting (opt-in)
- Feature usage analytics

### Privacy First
- No personal data collection
- No model content logging
- Local-only processing by default
- Clear opt-in mechanisms

## Success Metrics

### User Experience
- Time from download to first result: <5 minutes
- User satisfaction: High ease of use ratings
- Adoption rate: Community uptake

### Technical Performance
- Binary size: <15MB
- Memory usage: <10MB base
- Startup time: <100ms
- Benchmark accuracy: ±2% variance

## Future Roadmap

### Version 1.1
- GUI wrapper (optional)
- Prometheus metrics export
- Docker container support

### Version 1.2
- Plugin system
- Custom metric definitions
- Distributed benchmarking

### Version 2.0
- Web dashboard
- Historical trending
- Team collaboration features

---

This specification provides a comprehensive foundation for developing ollama-bench as a best-in-class Ollama benchmarking tool that fills the current market gap.