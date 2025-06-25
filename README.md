# ollama-meter

⚡ Apache Bench-style Ollama LLM performance benchmarking tool

## 🚀 Quick Start

```bash
# Install
cargo install --path .

# Basic usage - benchmark a single model
ollama-meter llama2:7b

# Compare multiple models
ollama-meter llama2:7b mistral:7b phi-2
```

## 📊 Features

- **Zero Dependencies** - Only requires Ollama to be installed
- **5-Minute Benchmark** - Get results fast with minimal setup  
- **Beautiful Output** - Progress bars and formatted tables
- **Multiple Formats** - Export as JSON, CSV, or Markdown
- **Cross-Platform** - Native support for Windows, macOS, and Linux
- **User-Friendly** - Clear error messages with helpful suggestions

## 📦 Installation

### From Source

```bash
git clone https://github.com/yourusername/ollama-meter
cd ollama-meter
cargo build --release
# Binary will be at ./target/release/ollama-meter
```

### Prerequisites

- [Rust](https://rustup.rs/) (1.70.0 or later)
- [Ollama](https://ollama.ai/) running locally

## 🎯 Usage

### Basic Benchmarking

```bash
# Benchmark a single model with default settings
ollama-meter llama2:7b

# Run 10 iterations instead of default 5
ollama-meter -n 10 llama2:7b

# Use a custom prompt
ollama-meter --prompt "Explain quantum computing" llama2:7b
```

### Comparing Models

```bash
# Compare multiple models
ollama-meter llama2:7b mistral:7b phi-2

# Export results to CSV
ollama-meter -e results.csv llama2:7b mistral:7b

# Output as JSON
ollama-meter -o json llama2:7b mistral:7b
```

### Advanced Options

```bash
# Full option list
ollama-meter --help

# Quiet mode (no progress bars)
ollama-meter -q llama2:7b

# Custom Ollama URL
ollama-meter --ollama-url http://remote:11434 llama2:7b

# Adjust generation parameters
ollama-meter -t 0.8 -m 200 llama2:7b  # temperature 0.8, max 200 tokens
```

## 📈 Output Example

```
⚡ ollama-meter v0.1.0 - Benchmarking 2 models with 5 iterations each

Testing llama2:7b...
████████████████████████████████ 100% (5/5)

Testing mistral:7b...
████████████████████████████████ 100% (5/5)

┌─────────────┬─────────────┬─────────────┬──────────────┐
│ Model       │ Avg Speed   │ TTFT        │ Success      │
├─────────────┼─────────────┼─────────────┼──────────────┤
│ llama2:7b   │ 28.5 tok/s  │ 234ms       │ 100%         │
│ mistral:7b  │ 31.2 tok/s  │ 198ms       │ 100%         │
└─────────────┴─────────────┴─────────────┴──────────────┘

🏆 Winner: mistral:7b (9.5% faster, 15% lower TTFT)
📊 Completed in 4m 13s
```

## 🔧 Configuration

### Environment Variables

- `OLLAMA_HOST` - Override default Ollama URL (default: http://localhost:11434)

### Output Formats

- **table** (default) - Beautiful ASCII table
- **json** - Structured JSON output
- **csv** - Comma-separated values
- **markdown** - Markdown table format

## 🏗️ Building from Source

```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy
```

## 🎯 Design Philosophy

ollama-meter follows the "Apache Bench" philosophy:

- **Simple** - One command to get started
- **Fast** - Minimal overhead, quick results
- **Clear** - Obvious what the results mean
- **Portable** - Works everywhere Ollama works

## 📊 Metrics Explained

- **Avg Speed** - Average tokens generated per second
- **TTFT** - Time To First Token (response latency)
- **Success Rate** - Percentage of successful completions

## 🐛 Troubleshooting

### "Ollama is not running"
```bash
# Start Ollama
ollama serve
```

### "Model not found"
```bash
# Pull the model first
ollama pull llama2:7b
```

### Performance tips
- Close other applications using GPU
- Ensure adequate RAM for model size
- Use smaller models for faster benchmarks

## 📝 License

MIT License

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

Built as a simpler alternative to existing enterprise-focused benchmarking tools, focusing on individual developer needs.