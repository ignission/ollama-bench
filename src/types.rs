use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaGenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub options: Option<OllamaOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaOptions {
    pub temperature: Option<f32>,
    pub num_predict: Option<i32>,
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaGenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub context: Option<Vec<i32>>,
    pub total_duration: Option<i64>,
    pub load_duration: Option<i64>,
    pub prompt_eval_count: Option<i32>,
    pub prompt_eval_duration: Option<i64>,
    pub eval_count: Option<i32>,
    pub eval_duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: String,
    pub size: i64,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModelsList {
    pub models: Vec<OllamaModel>,
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: u32,
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub timeout_seconds: u64,
    pub ollama_base_url: String,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 5,
            prompt: "Write a haiku about benchmarking language models.".to_string(),
            temperature: 0.7,
            max_tokens: 100,
            timeout_seconds: 120,
            ollama_base_url: "http://localhost:11434".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BenchmarkProgress {
    pub current_model: String,
    pub current_iteration: u32,
    pub total_iterations: u32,
    pub models_completed: u32,
    pub total_models: u32,
}

impl ModelSummary {
    pub fn from_results(model: String, results: &[BenchmarkResult]) -> Self {
        let successful_results: Vec<&BenchmarkResult> = results
            .iter()
            .filter(|r| r.success)
            .collect();
        
        let total_tests = results.len() as u32;
        let success_rate = if total_tests > 0 {
            successful_results.len() as f64 / total_tests as f64
        } else {
            0.0
        };
        
        let speeds: Vec<f64> = successful_results
            .iter()
            .map(|r| r.tokens_per_second)
            .collect();
        
        let ttfts: Vec<f64> = successful_results
            .iter()
            .map(|r| r.time_to_first_token_ms as f64)
            .collect();
        
        let avg_tokens_per_second = if !speeds.is_empty() {
            speeds.iter().sum::<f64>() / speeds.len() as f64
        } else {
            0.0
        };
        
        let min_tokens_per_second = speeds.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_tokens_per_second = speeds.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        let avg_ttft_ms = if !ttfts.is_empty() {
            ttfts.iter().sum::<f64>() / ttfts.len() as f64
        } else {
            0.0
        };
        
        Self {
            model,
            total_tests,
            success_rate,
            avg_tokens_per_second,
            min_tokens_per_second: if min_tokens_per_second.is_infinite() { 0.0 } else { min_tokens_per_second },
            max_tokens_per_second: if max_tokens_per_second.is_infinite() { 0.0 } else { max_tokens_per_second },
            avg_ttft_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_model_summary_from_results() {
        let results = vec![
            BenchmarkResult {
                model: "test-model".to_string(),
                prompt: "test".to_string(),
                timestamp: Utc::now(),
                success: true,
                tokens_per_second: 25.0,
                time_to_first_token_ms: 200,
                total_duration_ms: 1000,
                prompt_tokens: 10,
                completion_tokens: 25,
                error: None,
            },
            BenchmarkResult {
                model: "test-model".to_string(),
                prompt: "test".to_string(),
                timestamp: Utc::now(),
                success: true,
                tokens_per_second: 30.0,
                time_to_first_token_ms: 150,
                total_duration_ms: 900,
                prompt_tokens: 10,
                completion_tokens: 27,
                error: None,
            },
            BenchmarkResult {
                model: "test-model".to_string(),
                prompt: "test".to_string(),
                timestamp: Utc::now(),
                success: false,
                tokens_per_second: 0.0,
                time_to_first_token_ms: 0,
                total_duration_ms: 0,
                prompt_tokens: 0,
                completion_tokens: 0,
                error: Some("Failed".to_string()),
            },
        ];
        
        let summary = ModelSummary::from_results("test-model".to_string(), &results);
        
        assert_eq!(summary.total_tests, 3);
        assert_eq!(summary.success_rate, 2.0 / 3.0);
        assert_eq!(summary.avg_tokens_per_second, 27.5);
        assert_eq!(summary.min_tokens_per_second, 25.0);
        assert_eq!(summary.max_tokens_per_second, 30.0);
        assert_eq!(summary.avg_ttft_ms, 175.0);
    }
    
    #[test]
    fn test_benchmark_config_default() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.iterations, 5);
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, 100);
        assert_eq!(config.timeout_seconds, 120);
        assert_eq!(config.ollama_base_url, "http://localhost:11434");
    }
}