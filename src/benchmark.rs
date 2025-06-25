use std::time::Duration;
use tokio::time::sleep;

use crate::types::*;
use crate::error::Result;
use crate::ollama::OllamaClient;
use crate::progress::ProgressReporter;

pub struct Benchmarker {
    client: OllamaClient,
    config: BenchmarkConfig,
    progress: Box<dyn ProgressReporter>,
}

impl Benchmarker {
    pub fn new(
        client: OllamaClient,
        config: BenchmarkConfig,
        progress: Box<dyn ProgressReporter>,
    ) -> Self {
        Self {
            client,
            config,
            progress,
        }
    }
    
    pub async fn benchmark_models(&mut self, models: Vec<String>) -> Result<Vec<ModelSummary>> {
        let total_models = models.len() as u32;
        let mut all_results = Vec::new();
        
        // First, validate all models exist
        self.progress.print_info("Validating models...");
        for model in &models {
            if !self.client.validate_model(model).await? {
                return Err(crate::error::BenchmarkError::ModelNotFound(model.clone()));
            }
        }
        
        // Benchmark each model
        for (idx, model) in models.iter().enumerate() {
            let model_results = self.benchmark_single_model(
                model,
                idx as u32,
                total_models
            ).await?;
            
            all_results.push((model.clone(), model_results));
            
            // Small delay between models
            if idx < models.len() - 1 {
                sleep(Duration::from_millis(500)).await;
            }
        }
        
        // Generate summaries
        let summaries: Vec<ModelSummary> = all_results
            .into_iter()
            .map(|(model, results)| ModelSummary::from_results(model, &results))
            .collect();
        
        Ok(summaries)
    }
    
    async fn benchmark_single_model(
        &mut self,
        model: &str,
        model_index: u32,
        total_models: u32,
    ) -> Result<Vec<BenchmarkResult>> {
        let mut results = Vec::new();
        
        self.progress.start_model(model, model_index + 1, total_models);
        
        for iteration in 0..self.config.iterations {
            self.progress.update_progress(model, iteration + 1, self.config.iterations);
            
            let result = self.client.generate(
                model,
                &self.config.prompt,
                &self.config
            ).await?;
            
            results.push(result);
            
            // Small delay between iterations to avoid overwhelming the server
            if iteration < self.config.iterations - 1 {
                sleep(Duration::from_millis(100)).await;
            }
        }
        
        self.progress.complete_model(model);
        
        Ok(results)
    }
}

pub fn calculate_winner(summaries: &[ModelSummary]) -> Option<&ModelSummary> {
    if summaries.is_empty() {
        return None;
    }
    
    // Find the model with highest average tokens per second
    summaries
        .iter()
        .filter(|s| s.success_rate > 0.0)
        .max_by(|a, b| {
            a.avg_tokens_per_second
                .partial_cmp(&b.avg_tokens_per_second)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

pub fn calculate_performance_difference(winner: &ModelSummary, other: &ModelSummary) -> (f64, f64) {
    let speed_diff = if other.avg_tokens_per_second > 0.0 {
        ((winner.avg_tokens_per_second - other.avg_tokens_per_second) / other.avg_tokens_per_second) * 100.0
    } else {
        0.0
    };
    
    let ttft_diff = if other.avg_ttft_ms > 0.0 {
        ((other.avg_ttft_ms - winner.avg_ttft_ms) / other.avg_ttft_ms) * 100.0
    } else {
        0.0
    };
    
    (speed_diff, ttft_diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_winner() {
        let summaries = vec![
            ModelSummary {
                model: "model1".to_string(),
                total_tests: 5,
                success_rate: 1.0,
                avg_tokens_per_second: 25.0,
                min_tokens_per_second: 20.0,
                max_tokens_per_second: 30.0,
                avg_ttft_ms: 200.0,
            },
            ModelSummary {
                model: "model2".to_string(),
                total_tests: 5,
                success_rate: 1.0,
                avg_tokens_per_second: 30.0,
                min_tokens_per_second: 25.0,
                max_tokens_per_second: 35.0,
                avg_ttft_ms: 150.0,
            },
        ];
        
        let winner = calculate_winner(&summaries);
        assert!(winner.is_some());
        assert_eq!(winner.unwrap().model, "model2");
    }
    
    #[test]
    fn test_calculate_performance_difference() {
        let winner = ModelSummary {
            model: "winner".to_string(),
            total_tests: 5,
            success_rate: 1.0,
            avg_tokens_per_second: 30.0,
            min_tokens_per_second: 25.0,
            max_tokens_per_second: 35.0,
            avg_ttft_ms: 150.0,
        };
        
        let other = ModelSummary {
            model: "other".to_string(),
            total_tests: 5,
            success_rate: 1.0,
            avg_tokens_per_second: 25.0,
            min_tokens_per_second: 20.0,
            max_tokens_per_second: 30.0,
            avg_ttft_ms: 200.0,
        };
        
        let (speed_diff, ttft_diff) = calculate_performance_difference(&winner, &other);
        assert_eq!(speed_diff, 20.0); // 30 is 20% faster than 25
        assert_eq!(ttft_diff, 25.0); // 150ms is 25% lower than 200ms
    }
}