# Claude Code Development Task Checklist

> **Last Updated**: 2025-06-25  
> **Current Status**: MVP in development - Core structure complete, finishing implementation

## 📊 Implementation Progress Summary

### Overall Status: **~60% Complete** 🟡

#### ✅ Completed Components (5/13 files)
- `src/types.rs` - All data structures ✅
- `src/error.rs` - Error handling ✅
- `src/config.rs` - Configuration ✅
- `src/cli.rs` - CLI interface ✅
- `src/main.rs` - Entry point ✅

#### ⚠️ Partially Complete (5/13 files)
- `src/ollama.rs` - Missing `validate_model()` method
- `src/benchmark.rs` - Missing `calculate_performance_difference()`
- `src/runner.rs` - Incomplete `export_results()` method
- `src/output.rs` - Missing CSV completion and Markdown format
- `src/progress.rs` - Missing `QuietProgress` struct

#### ❌ Not Started (3/13 files)
- `src/validation.rs` - Input validation
- `src/utils.rs` - Utility functions
- `src/lib.rs` - Library structure (optional)

### 🚨 Priority Tasks to Complete MVP
1. Complete `generate()` method in `src/ollama.rs`
2. Add missing `validate_model()` method
3. Complete `calculate_winner()` and add `calculate_performance_difference()`
4. Finish CSV output and add Markdown format
5. Complete `export_results()` in runner
6. Add `QuietProgress` implementation
7. Create `src/validation.rs` for input validation

## Quick Start Instructions for Claude Code

### 🚀 Immediate First Steps

1. **Initialize Project**
   ```bash
   cargo new ollama-bench --bin
   cd ollama-bench
   ```

2. **Copy Cargo.toml** (from file structure document)

3. **Create src/lib.rs** for testing support:
   ```rust
   //! ollama-bench library for benchmarking Ollama models
   
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
- [x] **src/types.rs** - Implement all data structures from specification ✅
- [x] **src/error.rs** - Implement user-friendly error messages ✅
- [x] **src/config.rs** - Define constants and defaults ✅
- [x] **Acceptance**: `cargo check` passes ✅

#### Task 1.2: CLI Interface
- [x] **src/cli.rs** - Complete CLI argument parsing ✅
- [ ] **src/validation.rs** - Input validation with helpful errors
- [x] Test: `cargo run -- --help` shows proper help ✅
- [x] Test: `cargo run -- llama2:7b` parses correctly ✅
- [x] **Acceptance**: CLI parsing works for all argument combinations ✅

#### Task 1.3: Ollama Integration
- [~] **src/ollama.rs** - Implement OllamaClient with: ⚠️ Partially complete
  - [x] `health_check()` - Test Ollama connection ✅
  - [x] `list_models()` - Get available models ✅
  - [~] `generate()` - Send benchmark requests ⚠️ Incomplete
  - [x] Proper error handling for network issues ✅
  - [ ] `validate_model()` - Missing method referenced in benchmark.rs
- [~] **Acceptance**: Can connect to Ollama and get model list ⚠️ Partial

### ✅ Core Functionality (Essential Features)

#### Task 1.4: Benchmarking Engine
- [~] **src/benchmark.rs** - Core benchmarking logic: ⚠️ Partially complete
  - [x] Single model benchmarking ✅
  - [x] Timing measurement (TTFT, total duration) ✅
  - [x] Token counting and rate calculation ✅
  - [x] Error recovery and retry logic ✅
  - [~] `calculate_winner()` - Started but incomplete
  - [ ] `calculate_performance_difference()` - Missing function
- [~] **Acceptance**: Can benchmark one model successfully ⚠️ Partial

#### Task 1.5: Progress and Output
- [~] **src/progress.rs** - Terminal progress indicators: ⚠️ Partially complete
  - [x] Progress bar for each model ✅
  - [x] Real-time updates ✅
  - [x] Cross-platform terminal handling ✅
  - [ ] `QuietProgress` struct - Missing implementation
- [~] **src/output.rs** - Result formatting: ⚠️ Partially complete
  - [x] Beautiful table output with colors ✅
  - [x] JSON output format ✅
  - [~] CSV output format ⚠️ Started but incomplete
  - [ ] Markdown output format - Missing implementation
- [~] **Acceptance**: Shows progress and formats results nicely ⚠️ Partial

#### Task 1.6: Main Orchestration
- [~] **src/runner.rs** - Main execution logic: ⚠️ Partially complete
  - [x] Coordinate all components ✅
  - [x] Handle multiple models ✅
  - [x] Aggregate results and find winner ✅
  - [x] Error handling and recovery ✅
  - [~] `export_results()` - Method incomplete
- [x] **src/main.rs** - Entry point and CLI integration ✅
- [~] **Acceptance**: Complete end-to-end functionality ⚠️ Partial

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
- [~] `ollama-bench llama2:7b` works end-to-end ⚠️ Needs completion
- [~] `ollama-bench llama2:7b mistral:7b` compares models ⚠️ Needs completion
- [x] Beautiful table output with colors ✅
- [x] User-friendly error messages ✅
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
- **Binary size**: <15MB (use `ls -la target/release/ollama-bench`)
- **Startup time**: <100ms (use `time ./ollama-bench --help`)
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

This checklist provides Claude Code with a clear, prioritized roadmap for developing ollama-bench from initial concept to production-ready tool.