use clap::{Parser, ValueEnum};
use crate::config::*;

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = APP_DESCRIPTION)]
#[command(version = APP_VERSION)]
#[command(author)]
#[command(
    help_template = "{before-help}{name} {version}
{about}

{usage-heading} {usage}

{all-args}{after-help}

EXAMPLES:
    # Benchmark a single model
    {bin} llama2:7b

    # Compare multiple models
    {bin} llama2:7b mistral:7b phi-2

    # Custom iterations
    {bin} -n 10 llama2:7b

    # JSON output
    {bin} -o json llama2:7b mistral:7b

    # Custom prompt
    {bin} --prompt \"Explain quantum computing\" llama2:7b
"
)]
pub struct Cli {
    /// Models to benchmark (e.g., llama2:7b mistral:7b)
    #[arg(required = true, value_name = "MODEL")]
    pub models: Vec<String>,
    
    /// Number of test iterations per model
    #[arg(short = 'n', long, default_value_t = DEFAULT_ITERATIONS, value_name = "COUNT")]
    pub iterations: u32,
    
    /// Output format
    #[arg(short, long, default_value = "table", value_name = "FORMAT")]
    pub output: OutputFormat,
    
    /// Custom prompt for benchmarking
    #[arg(short, long, value_name = "TEXT")]
    pub prompt: Option<String>,
    
    /// Maximum tokens to generate
    #[arg(short = 'm', long, default_value_t = DEFAULT_MAX_TOKENS, value_name = "COUNT")]
    pub max_tokens: i32,
    
    /// Temperature for generation
    #[arg(short = 't', long, default_value_t = DEFAULT_TEMPERATURE, value_name = "FLOAT")]
    pub temperature: f32,
    
    /// Request timeout in seconds
    #[arg(long, default_value_t = DEFAULT_TIMEOUT_SECONDS, value_name = "SECONDS")]
    pub timeout: u64,
    
    /// Ollama API base URL
    #[arg(long, default_value = DEFAULT_OLLAMA_BASE_URL, value_name = "URL")]
    pub ollama_url: String,
    
    /// Quiet mode (no progress indicators)
    #[arg(short, long)]
    pub quiet: bool,
    
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Export results to file
    #[arg(short = 'e', long, value_name = "PATH")]
    pub export: Option<String>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    /// Pretty table output (default)
    Table,
    /// JSON output
    Json,
    /// CSV output
    Csv,
    /// Markdown table output
    Markdown,
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        // Validate iterations
        if self.iterations == 0 {
            return Err("Iterations must be greater than 0".to_string());
        }
        
        if self.iterations > 1000 {
            return Err("Iterations must be 1000 or less".to_string());
        }
        
        // Validate temperature
        if self.temperature < 0.0 || self.temperature > 2.0 {
            return Err("Temperature must be between 0.0 and 2.0".to_string());
        }
        
        // Validate max_tokens
        if self.max_tokens <= 0 {
            return Err("Max tokens must be greater than 0".to_string());
        }
        
        if self.max_tokens > 4096 {
            return Err("Max tokens must be 4096 or less".to_string());
        }
        
        // Validate timeout
        if self.timeout == 0 {
            return Err("Timeout must be greater than 0".to_string());
        }
        
        // Validate models
        if self.models.is_empty() {
            return Err("At least one model must be specified".to_string());
        }
        
        // Validate Ollama URL
        if !self.ollama_url.starts_with("http://") && !self.ollama_url.starts_with("https://") {
            return Err("Ollama URL must start with http:// or https://".to_string());
        }
        
        Ok(())
    }
    
    pub fn get_prompt(&self) -> String {
        self.prompt.as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| DEFAULT_PROMPT.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_validation_valid() {
        let cli = Cli {
            models: vec!["llama2:7b".to_string()],
            iterations: 5,
            output: OutputFormat::Table,
            prompt: None,
            max_tokens: 100,
            temperature: 0.7,
            timeout: 120,
            ollama_url: "http://localhost:11434".to_string(),
            quiet: false,
            verbose: false,
            export: None,
        };
        
        assert!(cli.validate().is_ok());
    }
    
    #[test]
    fn test_cli_validation_invalid_iterations() {
        let mut cli = Cli {
            models: vec!["llama2:7b".to_string()],
            iterations: 0,
            output: OutputFormat::Table,
            prompt: None,
            max_tokens: 100,
            temperature: 0.7,
            timeout: 120,
            ollama_url: "http://localhost:11434".to_string(),
            quiet: false,
            verbose: false,
            export: None,
        };
        
        assert!(cli.validate().is_err());
        
        cli.iterations = 1001;
        assert!(cli.validate().is_err());
    }
    
    #[test]
    fn test_cli_validation_invalid_temperature() {
        let mut cli = Cli {
            models: vec!["llama2:7b".to_string()],
            iterations: 5,
            output: OutputFormat::Table,
            prompt: None,
            max_tokens: 100,
            temperature: -0.1,
            timeout: 120,
            ollama_url: "http://localhost:11434".to_string(),
            quiet: false,
            verbose: false,
            export: None,
        };
        
        assert!(cli.validate().is_err());
        
        cli.temperature = 2.1;
        assert!(cli.validate().is_err());
    }
    
    #[test]
    fn test_get_prompt() {
        let mut cli = Cli {
            models: vec!["llama2:7b".to_string()],
            iterations: 5,
            output: OutputFormat::Table,
            prompt: None,
            max_tokens: 100,
            temperature: 0.7,
            timeout: 120,
            ollama_url: "http://localhost:11434".to_string(),
            quiet: false,
            verbose: false,
            export: None,
        };
        
        assert_eq!(cli.get_prompt(), DEFAULT_PROMPT);
        
        cli.prompt = Some("Custom prompt".to_string());
        assert_eq!(cli.get_prompt(), "Custom prompt");
    }
}