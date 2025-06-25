use std::fmt;

#[derive(Debug)]
pub enum BenchmarkError {
    OllamaNotRunning,
    ModelNotFound(String),
    NetworkTimeout(u64),
    InvalidModel(String),
    ConnectionFailed(String),
    ParseError(String),
    IoError(String),
    ConfigError(String),
}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BenchmarkError::OllamaNotRunning => {
                write!(f, "❌ Ollama is not running\n💡 Start with: ollama serve")
            }
            BenchmarkError::ModelNotFound(model) => {
                write!(f, "❌ Model '{}' not found\n💡 Install with: ollama pull {}", model, model)
            }
            BenchmarkError::NetworkTimeout(seconds) => {
                write!(f, "❌ Network timeout after {}s\n💡 Try increasing --timeout", seconds)
            }
            BenchmarkError::InvalidModel(model) => {
                write!(f, "❌ Invalid model name: '{}'\n💡 Model names should be in format: model:tag (e.g., llama2:7b)", model)
            }
            BenchmarkError::ConnectionFailed(url) => {
                write!(f, "❌ Failed to connect to Ollama at {}\n💡 Check if Ollama is running and accessible", url)
            }
            BenchmarkError::ParseError(msg) => {
                write!(f, "❌ Failed to parse response: {}\n💡 This might be a compatibility issue with your Ollama version", msg)
            }
            BenchmarkError::IoError(msg) => {
                write!(f, "❌ I/O error: {}\n💡 Check file permissions and disk space", msg)
            }
            BenchmarkError::ConfigError(msg) => {
                write!(f, "❌ Configuration error: {}\n💡 {}", msg, msg)
            }
        }
    }
}

impl std::error::Error for BenchmarkError {}

impl From<std::io::Error> for BenchmarkError {
    fn from(error: std::io::Error) -> Self {
        BenchmarkError::IoError(error.to_string())
    }
}

impl From<reqwest::Error> for BenchmarkError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_connect() {
            BenchmarkError::OllamaNotRunning
        } else if error.is_timeout() {
            BenchmarkError::NetworkTimeout(30) // Default timeout
        } else {
            BenchmarkError::ConnectionFailed(error.to_string())
        }
    }
}

impl From<serde_json::Error> for BenchmarkError {
    fn from(error: serde_json::Error) -> Self {
        BenchmarkError::ParseError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, BenchmarkError>;

pub fn validate_model_name(model: &str) -> Result<()> {
    if model.is_empty() {
        return Err(BenchmarkError::InvalidModel("empty model name".to_string()));
    }
    
    // Basic validation - model names typically contain alphanumeric characters, colons, and hyphens
    if !model.chars().all(|c| c.is_alphanumeric() || c == ':' || c == '-' || c == '_' || c == '.') {
        return Err(BenchmarkError::InvalidModel(model.to_string()));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = BenchmarkError::OllamaNotRunning;
        assert!(err.to_string().contains("ollama serve"));
        
        let err = BenchmarkError::ModelNotFound("llama2:7b".to_string());
        assert!(err.to_string().contains("ollama pull llama2:7b"));
        
        let err = BenchmarkError::NetworkTimeout(60);
        assert!(err.to_string().contains("60s"));
    }
    
    #[test]
    fn test_validate_model_name() {
        assert!(validate_model_name("llama2:7b").is_ok());
        assert!(validate_model_name("mistral:latest").is_ok());
        assert!(validate_model_name("phi-2").is_ok());
        assert!(validate_model_name("").is_err());
        assert!(validate_model_name("model with spaces").is_err());
        assert!(validate_model_name("model@invalid").is_err());
    }
}