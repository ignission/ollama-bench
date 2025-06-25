# Claude Code Development Task Checklist

## Quick Start Instructions for Claude Code

### 🚀 Immediate First Steps

1. **Initialize Project**
   ```bash
   cargo new ollama-meter --bin
   cd ollama-meter
   ```

2. **Copy Cargo.toml** (from file structure document)

3. **Create src/lib.rs** for testing support:
   ```rust
   //! ollama-meter library for benchmarking Ollama models
   
   pub mod types;
   pub mod error;
   pub mod config;
   pub mod cli;
   pub mod ollama;
   pub mod benchmark;
   pub mod runner;
   pub mod output;
   pub mod progress;
   pub mod validation;
   pub mod utils;
   ```

## Phase 1: MVP Development (Essential for Working Tool)

### ✅ Foundation Files (Complete First)

#### Task 1.1: Core Types and Errors
- [ ] **src/types.rs** - Implement all data structures from specification
- [ ] **src/error.rs** - Implement user-friendly error messages
- [ ] **src/config.rs** - Define constants and defaults
- [ ] **Acceptance**: `cargo check` passes

#### Task 1.2: CLI Interface
- [ ] **src/cli.rs** - Complete CLI argument parsing
- [ ] **src/validation.rs** - Input validation with helpful errors
- [ ] Test: `cargo run -- --help` shows proper help
- [ ] Test: `cargo run -- llama2:7b` parses correctly
- [ ] **Acceptance**: CLI parsing works for all argument combinations

#### Task 1.3: Ollama Integration
- [ ] **src/ollama.rs** - Implement OllamaClient with:
  - [ ] `health_check()` - Test Ollama connection
  - [ ] `list_models()` - Get available models
  - [ ] `generate()` - Send benchmark requests
  - [ ] Proper error handling for network issues
- [ ] **Acceptance**: Can connect to Ollama and get model list

### ✅ Core Functionality (Essential Features)

#### Task 1.4: Benchmarking Engine
- [ ] **src/benchmark.rs** - Core benchmarking logic:
  - [ ] Single model benchmarking
  - [ ] Timing measurement (TTFT, total duration)
  - [ ] Token counting and rate calculation
  - [ ] Error recovery and retry logic
- [ ] **Acceptance**: Can benchmark one model successfully

#### Task 1.5: Progress and Output
- [ ] **src/progress.rs** - Terminal progress indicators:
  - [ ] Progress bar for each model
  - [ ] Real-time updates
  - [ ] Cross-platform terminal handling
- [ ] **src/output.rs** - Result formatting:
  - [ ] Beautiful table output with colors
  - [ ] JSON output format
  - [ ] CSV output format
- [ ] **Acceptance**: Shows progress and formats results nicely

#### Task 1.6: Main Orchestration
- [ ] **src/runner.rs** - Main execution logic:
  - [ ] Coordinate all components
  - [ ] Handle multiple models
  - [ ] Aggregate results and find winner
  - [ ] Error handling and recovery
- [ ] **src/main.rs** - Entry point and CLI integration
- [ ] **Acceptance**: Complete end-to-end functionality

## Phase 2: Polish and Reliability (Production Ready)

### ✅ Error Handling and Edge Cases

#### Task 2.1: Robust Error Handling
- [ ] Handle Ollama not running (with helpful message)
- [ ] Handle model not found (suggest installation)
- [ ] Handle network timeouts gracefully
- [ ] Handle invalid responses from Ollama
- [ ] Graceful degradation (continue with other models if one fails)
- [ ] **Acceptance**: All error scenarios show helpful messages

#### Task 2.2: Input Validation
- [ ] Validate model names exist before starting
- [ ] Validate Ollama server accessibility
- [ ] Validate output file paths and permissions
- [ ] Validate prompt format and length
- [ ] **Acceptance**: Prevents common user errors with clear guidance

### ✅ Testing Infrastructure

#### Task 2.3: Unit Tests
- [ ] **src/types.rs** - Test serialization/deserialization
- [ ] **src/benchmark.rs** - Test calculations and timing
- [ ] **src/output.rs** - Test formatting functions
- [ ] **src/validation.rs** - Test validation logic
- [ ] **Acceptance**: `cargo test` passes with >80% coverage

#### Task 2.4: Integration Tests
- [ ] **tests/integration_tests.rs** - End-to-end CLI testing
- [ ] **tests/cli_tests.rs** - CLI argument parsing tests
- [ ] Mock Ollama responses for testing
- [ ] **Acceptance**: Integration tests pass without real Ollama server

## Phase 3: User Experience Enhancement

### ✅ Advanced Features

#### Task 3.1: Enhanced Output
- [ ] **src/html.rs** - HTML report generation
- [ ] Beautiful comparison charts
- [ ] Responsive design for sharing
- [ ] **Acceptance**: HTML reports are visually appealing

#### Task 3.2: Advanced CLI Features
- [ ] Configuration file support
- [ ] Custom prompt templates
- [ ] Concurrent request testing
- [ ] Historical comparison
- [ ] **Acceptance**: Advanced users can customize all aspects

### ✅ Documentation and Distribution

#### Task 3.3: Documentation
- [ ] **README.md** - Clear installation and usage guide
- [ ] **CONTRIBUTING.md** - Development guidelines
- [ ] **CHANGELOG.md** - Version history
- [ ] **docs/API.md** - Detailed API documentation
- [ ] **Acceptance**: New users can get started in <5 minutes

#### Task 3.4: Build and Release
- [ ] **.github/workflows/ci.yml** - Automated testing
- [ ] **.github/workflows/release.yml** - Cross-platform builds
- [ ] **scripts/build_release.sh** - Release build script
- [ ] **Acceptance**: Automated builds for all platforms

## Quality Gates and Acceptance Criteria

### 🎯 MVP Release Criteria (v0.1.0)
- [ ] `ollama-meter llama2:7b` works end-to-end
- [ ] `ollama-meter llama2:7b mistral:7b` compares models
- [ ] Beautiful table output with colors
- [ ] User-friendly error messages
- [ ] Cross-platform binary <15MB
- [ ] Startup time <100ms
- [ ] 5-minute first-run experience

### 🎯 Production Release Criteria (v1.0.0)
- [ ] All Phase 1 and Phase 2 tasks complete
- [ ] Comprehensive test suite (unit + integration)
- [ ] Documentation complete
- [ ] No clippy warnings
- [ ] Code coverage >80%
- [ ] Performance targets met
- [ ] Cross-platform testing passed

## Implementation Tips for Claude Code

### 🔧 Development Workflow
1. **Start with types.rs** - Define all data structures first
2. **Build incrementally** - Get each component working before moving on
3. **Test immediately** - Add tests as you implement features
4. **Run frequently** - Use `cargo check`, `cargo test`, `cargo run`
5. **Handle errors early** - Implement proper error handling from the start

### 🚨 Critical Success Factors
1. **Focus on user experience** - Every error message should be helpful
2. **Keep it simple** - Apache Bench-style simplicity is the goal
3. **Performance matters** - Fast startup and execution
4. **Cross-platform compatibility** - Test on multiple platforms
5. **Beautiful output** - Terminal output should be visually appealing

### 📋 Before Each Commit Checklist
- [ ] `cargo check` passes
- [ ] `cargo test` passes  
- [ ] `cargo clippy` shows no warnings
- [ ] `cargo fmt` applied
- [ ] Manual testing works
- [ ] Error messages are user-friendly

### 🎯 Success Metrics
- **Binary size**: <15MB (use `ls -la target/release/ollama-meter`)
- **Startup time**: <100ms (use `time ./ollama-meter --help`)
- **Memory usage**: <10MB base (use `ps` or Activity Monitor)
- **First-run experience**: <5 minutes from download to results

## Final Delivery Requirements

### 📦 MVP Deliverables
1. **Working binary** for at least one platform
2. **Source code** with all Phase 1 tasks complete
3. **Basic README** with installation and usage
4. **Test suite** with core functionality covered
5. **Example output** showing the tool in action

### 🏆 Production Deliverables
1. **Cross-platform binaries** (Windows, macOS, Linux)
2. **Complete source code** with all phases
3. **Comprehensive documentation**
4. **Full test suite** with CI/CD
5. **Release automation** and distribution setup

---

This checklist provides Claude Code with a clear, prioritized roadmap for developing ollama-meter from initial concept to production-ready tool.