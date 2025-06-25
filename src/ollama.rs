use std::time::{Duration, Instant};
use reqwest::Client;
use serde_json::json;
use chrono::Utc;

use crate::types::*;
use crate::error::{BenchmarkError, Result};
use crate::config::get_user_agent;

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String, timeout: Duration) -> Self {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(get_user_agent())
            .build()
            .unwrap_or_default();
            
        Self { client, base_url }
    }
    
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                if e.is_connect() {
                    Err(BenchmarkError::OllamaNotRunning)
                } else {
                    Err(BenchmarkError::ConnectionFailed(self.base_url.clone()))
                }
            }
        }
    }
    
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(BenchmarkError::ConnectionFailed(
                format!("HTTP {} from Ollama", response.status())
            ));
        }
        
        let models_list: OllamaModelsList = response.json().await?;
        Ok(models_list.models.into_iter().map(|m| m.name).collect())
    }
    
    pub async fn generate(&self, model: &str, prompt: &str, config: &BenchmarkConfig) -> Result<BenchmarkResult> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request_body = json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": config.temperature,
                "num_predict": config.max_tokens,
            }
        });
        
        let start_time = Instant::now();
        let timestamp = Utc::now();
        
        let response = match self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => {
                    return Ok(BenchmarkResult {
                        model: model.to_string(),
                        prompt: prompt.to_string(),
                        timestamp,
                        success: false,
                        tokens_per_second: 0.0,
                        time_to_first_token_ms: 0,
                        total_duration_ms: start_time.elapsed().as_millis() as u64,
                        prompt_tokens: 0,
                        completion_tokens: 0,
                        error: Some(e.to_string()),
                    });
                }
            };
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            // Check if it's a model not found error
            if status.as_u16() == 404 || error_text.contains("model") {
                return Err(BenchmarkError::ModelNotFound(model.to_string()));
            }
            
            return Ok(BenchmarkResult {
                model: model.to_string(),
                prompt: prompt.to_string(),
                timestamp,
                success: false,
                tokens_per_second: 0.0,
                time_to_first_token_ms: 0,
                total_duration_ms: start_time.elapsed().as_millis() as u64,
                prompt_tokens: 0,
                completion_tokens: 0,
                error: Some(format!("HTTP {}: {}", status, error_text)),
            });
        }
        
        let ollama_response: OllamaGenerateResponse = match response.json().await {
            Ok(resp) => resp,
            Err(e) => {
                return Ok(BenchmarkResult {
                    model: model.to_string(),
                    prompt: prompt.to_string(),
                    timestamp,
                    success: false,
                    tokens_per_second: 0.0,
                    time_to_first_token_ms: 0,
                    total_duration_ms: start_time.elapsed().as_millis() as u64,
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    error: Some(format!("Failed to parse response: {}", e)),
                });
            }
        };
        
        // Calculate metrics
        let total_duration_ms = start_time.elapsed().as_millis() as u64;
        
        // Extract timing information from Ollama response
        let prompt_eval_duration = ollama_response.prompt_eval_duration.unwrap_or(0);
        let eval_duration = ollama_response.eval_duration.unwrap_or(0);
        let prompt_tokens = ollama_response.prompt_eval_count.unwrap_or(0) as u32;
        let completion_tokens = ollama_response.eval_count.unwrap_or(0) as u32;
        
        // Calculate time to first token (approximation)
        let time_to_first_token_ms = if prompt_eval_duration > 0 {
            (prompt_eval_duration / 1_000_000) as u64 // Convert nanoseconds to milliseconds
        } else {
            0
        };
        
        // Calculate tokens per second
        let tokens_per_second = if eval_duration > 0 && completion_tokens > 0 {
            (completion_tokens as f64 * 1_000_000_000.0) / eval_duration as f64
        } else {
            0.0
        };
        
        Ok(BenchmarkResult {
            model: model.to_string(),
            prompt: prompt.to_string(),
            timestamp,
            success: true,
            tokens_per_second,
            time_to_first_token_ms,
            total_duration_ms,
            prompt_tokens,
            completion_tokens,
            error: None,
        })
    }
    
    pub async fn validate_model(&self, model: &str) -> Result<bool> {
        let models = self.list_models().await?;
        Ok(models.iter().any(|m| m == model))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_ollama_client_creation() {
        let client = OllamaClient::new(
            "http://localhost:11434".to_string(),
            Duration::from_secs(30)
        );
        assert_eq!(client.base_url, "http://localhost:11434");
    }
    
    #[tokio::test]
    async fn test_benchmark_result_on_error() {
        let client = OllamaClient::new(
            "http://invalid-url:11434".to_string(),
            Duration::from_secs(1)
        );
        
        let config = BenchmarkConfig::default();
        let result = client.generate("test-model", "test prompt", &config).await;
        
        match result {
            Ok(benchmark_result) => {
                assert!(!benchmark_result.success);
                assert!(benchmark_result.error.is_some());
                assert_eq!(benchmark_result.tokens_per_second, 0.0);
            }
            Err(_) => {
                // This is also acceptable - connection error
            }
        }
    }
}