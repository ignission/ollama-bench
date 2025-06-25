# CLAUDE.md - Master Instructions for ollama-meter Development

## 🎯 Project Mission

You are developing **ollama-meter**, a Rust CLI tool that solves the "5-minute benchmark" problem for Ollama LLM performance testing. This tool fills a critical market gap identified through comprehensive analysis.

## 🔍 Problem Context

**Market Analysis Findings:**
- Existing tools (cloudmercato/ollama-benchmark, mikl0s/OBT) are overengineered for enterprise use
- Individual developers need simple, Apache Bench-style performance comparison
- Current tools require 30+ minutes setup vs. our 5-minute guarantee
- No tools focus on Windows optimization or Japanese language support

## 🏆 Success Vision

Create the **"Apache Bench of Ollama benchmarking"** - a tool that:
- Works in <5 minutes from download to results
- Requires zero dependencies except Ollama
- Provides beautiful, clear output
- Runs natively on Windows, macOS, and Linux
- Becomes the go-to tool for individual developers

## 🚀 Immediate Action Plan

### Phase 1: MVP Development (Essential)

#### 🔥 Start Here - Core Implementation Order:

1. **Initialize Project Structure**
   ```bash
   cargo new ollama-meter --bin
   cd ollama-meter
   # Copy Cargo.toml from docs/FILE_STRUCTURE.md
   ```

2. **Implement Foundation (Priority 1)**
   - `src/types.rs` - All data structures (complete specification in docs/)
   - `src/error.rs` - User-friendly error messages
   - `src/config.rs` - Constants and defaults
   - `src/cli.rs` - Command-line interface

3. **Build Core Engine (Priority 2)**
   - `src/ollama.rs` - Ollama API client
   - `src/benchmark.rs` - Core benchmarking logic
   - `src/runner.rs` - Main execution orchestration

4. **Polish User Experience (Priority 3)**
   - `src/progress.rs` - Progress indicators
   - `src/output.rs` - Beautiful table formatting
   - `src/main.rs` - Entry point integration

#### ✅ MVP Success Criteria:
- `ollama-meter llama2:7b` works end-to-end
- `ollama-meter llama2:7b mistral:7b` compares models
- Beautiful colored table output
- User-friendly error messages
- <15MB binary, <100ms startup

### Phase 2: Production Polish

#### Enhanced Features:
- JSON/CSV output formats
- HTML report generation
- Concurrent request testing
- Configuration file support
- Comprehensive error handling

## 📚 Essential Documentation (READ THESE)

### 🎯 Primary Development Guide
**`docs/CLAUDE_CODE_INSTRUCTIONS.md`** - Detailed implementation instructions
- Complete code examples for each component
- Error handling patterns
- Testing strategies
- Build configuration

### 📋 Implementation Reference
**`docs/DEVELOPMENT_SPEC.md`** - Complete technical specification
- Architecture overview
- Data structure definitions
- CLI interface design
- Performance requirements

### 🏗️ Code Organization
**`docs/FILE_STRUCTURE.md`** - Project structure and file organization
- Complete directory layout
- File dependency order
- Implementation priorities
- Quality standards

### ✅ Progress Tracking
**`docs/TASK_CHECKLIST.md`** - Detailed task breakdown
- Phase-by-phase implementation plan
- Acceptance criteria for each task
- Quality gates and success metrics

## 🔧 Development Standards

### Code Quality Requirements:
- `cargo check` must pass
- `cargo test` must pass
- `cargo clippy` with no warnings
- `cargo fmt` applied
- All public APIs documented
- User-friendly error messages

### Performance Targets:
- Binary size: <15MB
- Startup time: <100ms  
- Memory usage: <10MB base
- 5-minute first-run experience

### Cross-Platform Requirements:
- Native Windows support (primary market gap)
- macOS (Intel + Apple Silicon)
- Linux (x86_64)

## 🎨 User Experience Design

### CLI Design Philosophy (Apache Bench Style):
```bash
# Simple as ab command
ab -n 1000 -c 10 http://example.com/

# Our equivalent
ollama-meter -n 5 llama2:7b mistral:7b
```

### Expected Output Style:
```
⚡ ollama-meter v1.0.0 - Benchmarking 2 models with 5 iterations each

Testing llama2:7b...
████████████████████████████████ 100% (5/5) [00:02:15]

Testing mistral:7b...
████████████████████████████████ 100% (5/5) [00:01:58]

┌─────────────┬─────────────┬─────────────┬──────────────┐
│ Model       │ Avg Speed   │ TTFT        │ Success      │
├─────────────┼─────────────┼─────────────┼──────────────┤
│ llama2:7b   │ 28.5 tok/s  │ 234ms       │ 100%         │
│ mistral:7b  │ 31.2 tok/s  │ 198ms       │ 100%         │
└─────────────┴─────────────┴─────────────┴──────────────┘

🏆 Winner: mistral:7b (9.5% faster, 15% lower TTFT)
📊 Completed in 4m 13s
```

## 🚨 Critical Success Factors

### 1. User-Friendly Error Messages
```rust
// Bad: "Connection refused"
// Good: "❌ Ollama is not running\n💡 Start with: ollama serve"
```

### 2. Fast Startup and Execution
- No unnecessary dependencies
- Efficient async implementation
- Minimal memory allocation

### 3. Beautiful Terminal Output
- Colored progress bars
- Clear table formatting
- Meaningful emoji indicators
- Cross-platform terminal compatibility

### 4. Graceful Error Handling
- Continue with other models if one fails
- Clear guidance for resolving issues
- Recoverable vs. fatal error distinction

## 🔍 Testing Strategy

### Essential Tests:
- Unit tests for all core logic
- Integration tests with mock Ollama responses
- CLI argument parsing validation
- Cross-platform compatibility testing
- Performance benchmarking

### Testing Commands:
```bash
cargo test                    # Unit tests
cargo test --test integration # Integration tests
cargo run -- --help          # CLI validation
cargo run -- llama2:7b       # End-to-end test
```

## 📦 Build and Distribution

### Release Targets:
```bash
# Cross-compilation targets
x86_64-pc-windows-msvc        # Windows
x86_64-apple-darwin           # macOS Intel
aarch64-apple-darwin          # macOS Apple Silicon  
x86_64-unknown-linux-gnu      # Linux
```

### Optimized Release Build:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

## 🎯 Market Opportunity

You're building a tool that addresses a **real market gap**:
- 15+ existing tools are overengineered for enterprise
- Individual developers need simplicity
- Windows optimization is largely ignored
- "Just works" experience is missing

**Success means**: Becoming the default tool individual developers reach for when they need quick LLM performance comparison.

## 🚀 Next Steps

1. **Start immediately** with `docs/CLAUDE_CODE_INSTRUCTIONS.md`
2. **Follow the task checklist** in `docs/TASK_CHECKLIST.md`
3. **Reference implementation details** in `docs/DEVELOPMENT_SPEC.md`
4. **Organize code** according to `docs/FILE_STRUCTURE.md`

## 💡 Development Tips

- **Build incrementally** - Get each component working before moving on
- **Test immediately** - Add tests as you implement features
- **Focus on UX** - Every error message should be helpful
- **Keep it simple** - Apache Bench-style simplicity is the goal
- **Performance matters** - Fast startup and execution are key differentiators

---

**Remember**: You're not just building a tool, you're solving a real problem for thousands of developers who are currently using complex enterprise tools or manual curl commands for simple performance comparisons.

Let's build something that developers will love to use! 🚀