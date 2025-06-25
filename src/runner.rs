use std::time::{Duration, Instant};
use std::fs::File;
use std::io::Write;

use crate::cli::{Cli, OutputFormat};
use crate::types::{BenchmarkConfig, ModelSummary};
use crate::error::{Result, BenchmarkError};
use crate::ollama::OllamaClient;
use crate::benchmark::{Benchmarker, calculate_winner, calculate_performance_difference};
use crate::progress::{ProgressReporter, TerminalProgress, QuietProgress};
use crate::output::{print_results_table, print_results_json, print_results_csv, print_results_markdown};

pub struct BenchmarkRunner {
    cli: Cli,
}

impl BenchmarkRunner {
    pub fn new(cli: Cli) -> Self {
        Self { cli }
    }
    
    pub async fn run(&self) -> Result<()> {
        // Validate CLI arguments
        self.cli.validate()
            .map_err(BenchmarkError::ConfigError)?;
        
        // Validate model names
        for model in &self.cli.models {
            crate::error::validate_model_name(model)?;
        }
        
        // Create configuration
        let config = BenchmarkConfig {
            iterations: self.cli.iterations,
            prompt: self.cli.get_prompt(),
            temperature: self.cli.temperature,
            max_tokens: self.cli.max_tokens,
            timeout_seconds: self.cli.timeout,
            ollama_base_url: self.cli.ollama_url.clone(),
        };
        
        // Create Ollama client
        let client = OllamaClient::new(
            config.ollama_base_url.clone(),
            Duration::from_secs(config.timeout_seconds),
        );
        
        // Check Ollama connectivity
        if !self.cli.quiet {
            println!("🔍 Checking Ollama connection...");
        }
        
        client.health_check().await?;
        
        // Create progress reporter
        let progress: Box<dyn ProgressReporter> = if self.cli.quiet {
            Box::new(QuietProgress)
        } else {
            Box::new(TerminalProgress::new(self.cli.quiet, self.cli.verbose))
        };
        
        // Create benchmarker
        let mut benchmarker = Benchmarker::new(client, config, progress);
        
        // Run benchmarks
        let start_time = Instant::now();
        let summaries = benchmarker.benchmark_models(self.cli.models.clone()).await?;
        let total_duration = start_time.elapsed();
        
        // Output results
        self.output_results(&summaries, total_duration)?;
        
        // Export if requested
        if let Some(export_path) = &self.cli.export {
            self.export_results(&summaries, export_path)?;
        }
        
        Ok(())
    }
    
    fn output_results(&self, summaries: &[ModelSummary], duration: Duration) -> Result<()> {
        match self.cli.output {
            OutputFormat::Table => {
                print_results_table(summaries, duration);
            }
            OutputFormat::Json => {
                print_results_json(summaries);
            }
            OutputFormat::Csv => {
                print_results_csv(summaries);
            }
            OutputFormat::Markdown => {
                print_results_markdown(summaries, duration);
            }
        }
        
        Ok(())
    }
    
    fn export_results(&self, summaries: &[ModelSummary], path: &str) -> Result<()> {
        let content = match path.rsplit('.').next() {
            Some("json") => serde_json::to_string_pretty(summaries)?,
            Some("csv") => self.generate_csv_content(summaries),
            Some("md") => self.generate_markdown_content(summaries),
            _ => {
                return Err(BenchmarkError::ConfigError(
                    "Export file must have .json, .csv, or .md extension".to_string()
                ));
            }
        };
        
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        
        if !self.cli.quiet {
            println!("📊 Results exported to: {}", path);
        }
        
        Ok(())
    }
    
    fn generate_csv_content(&self, summaries: &[ModelSummary]) -> String {
        let mut content = String::from("Model,Success Rate,Avg Tokens/s,Min Tokens/s,Max Tokens/s,Avg TTFT (ms)\n");
        
        for summary in summaries {
            content.push_str(&format!(
                "{},{:.1},{:.1},{:.1},{:.1},{:.0}\n",
                summary.model,
                summary.success_rate * 100.0,
                summary.avg_tokens_per_second,
                summary.min_tokens_per_second,
                summary.max_tokens_per_second,
                summary.avg_ttft_ms
            ));
        }
        
        content
    }
    
    fn generate_markdown_content(&self, summaries: &[ModelSummary]) -> String {
        let mut content = String::from("# Ollama Benchmark Results\n\n");
        content.push_str("| Model | Success Rate | Avg Tokens/s | TTFT (ms) |\n");
        content.push_str("|-------|--------------|--------------|------------|\n");
        
        for summary in summaries {
            content.push_str(&format!(
                "| {} | {:.1}% | {:.1} | {:.0} |\n",
                summary.model,
                summary.success_rate * 100.0,
                summary.avg_tokens_per_second,
                summary.avg_ttft_ms
            ));
        }
        
        if let Some(winner) = calculate_winner(summaries) {
            content.push_str(&format!("\n**Winner:** {} 🏆\n", winner.model));
            
            for other in summaries {
                if other.model != winner.model {
                    let (speed_diff, _ttft_diff) = calculate_performance_difference(winner, other);
                    if speed_diff > 0.0 {
                        content.push_str(&format!(
                            "- {:.1}% faster than {}\n",
                            speed_diff, other.model
                        ));
                    }
                }
            }
        }
        
        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::OutputFormat;

    #[test]
    fn test_generate_csv_content() {
        let cli = Cli {
            models: vec!["test".to_string()],
            iterations: 5,
            output: OutputFormat::Csv,
            prompt: None,
            max_tokens: 100,
            temperature: 0.7,
            timeout: 120,
            ollama_url: "http://localhost:11434".to_string(),
            quiet: false,
            verbose: false,
            export: None,
        };
        
        let runner = BenchmarkRunner::new(cli);
        
        let summaries = vec![
            ModelSummary {
                model: "test-model".to_string(),
                total_tests: 5,
                success_rate: 1.0,
                avg_tokens_per_second: 25.5,
                min_tokens_per_second: 20.0,
                max_tokens_per_second: 30.0,
                avg_ttft_ms: 200.0,
            }
        ];
        
        let csv = runner.generate_csv_content(&summaries);
        assert!(csv.contains("Model,Success Rate"));
        assert!(csv.contains("test-model,100.0,25.5"));
    }
}