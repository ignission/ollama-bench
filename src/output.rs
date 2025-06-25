use std::time::Duration;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use crate::types::ModelSummary;
use crate::benchmark::{calculate_winner, calculate_performance_difference};
use crate::config::TABLE_COLUMN_WIDTHS;

pub fn print_results_table(summaries: &[ModelSummary], duration: Duration) {
    if summaries.is_empty() {
        println!("\nNo results to display.");
        return;
    }
    
    println!("\n┌─────────────┬─────────────┬─────────────┬──────────────┐");
    println!("│ Model       │ Avg Speed   │ TTFT        │ Success      │");
    println!("├─────────────┼─────────────┼─────────────┼──────────────┤");
    
    for summary in summaries {
        let model_display = if summary.model.len() > TABLE_COLUMN_WIDTHS.model - 2 {
            format!("{}…", &summary.model[..TABLE_COLUMN_WIDTHS.model - 3])
        } else {
            summary.model.clone()
        };
        
        println!(
            "│ {:11} │ {:9.1} tok/s │ {:9.0}ms │ {:10.1}% │",
            model_display,
            summary.avg_tokens_per_second,
            summary.avg_ttft_ms,
            summary.success_rate * 100.0
        );
    }
    
    println!("└─────────────┴─────────────┴─────────────┴──────────────┘");
    
    // Print winner and comparison
    if summaries.len() > 1 {
        if let Some(winner) = calculate_winner(summaries) {
            execute!(
                std::io::stdout(),
                Print("\n"),
                SetForegroundColor(Color::Green),
                Print("🏆 Winner: "),
                Print(&winner.model),
                ResetColor
            ).ok();
            
            // Calculate and show performance differences
            let mut comparisons = Vec::new();
            for other in summaries {
                if other.model != winner.model && other.success_rate > 0.0 {
                    let (speed_diff, ttft_diff) = calculate_performance_difference(winner, other);
                    if speed_diff > 0.0 {
                        comparisons.push(format!("{:.1}% faster", speed_diff));
                    }
                    if ttft_diff > 0.0 && comparisons.len() < 2 {
                        comparisons.push(format!("{:.0}% lower TTFT", ttft_diff));
                    }
                }
            }
            
            if !comparisons.is_empty() {
                print!(" ({})", comparisons.join(", "));
            }
            println!();
        }
    }
    
    // Print completion time
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Cyan),
        Print("\n📊 Completed in "),
        ResetColor
    ).ok();
    
    if minutes > 0 {
        print!("{}m {}s", minutes, seconds);
    } else {
        print!("{}s", duration.as_secs());
    }
    println!();
}

pub fn print_results_json(summaries: &[ModelSummary]) {
    match serde_json::to_string_pretty(summaries) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing results: {}", e),
    }
}

pub fn print_results_csv(summaries: &[ModelSummary]) {
    println!("Model,Total Tests,Success Rate,Avg Tokens/s,Min Tokens/s,Max Tokens/s,Avg TTFT (ms)");
    
    for summary in summaries {
        println!(
            "{},{},{:.2},{:.2},{:.2},{:.2},{:.0}",
            summary.model,
            summary.total_tests,
            summary.success_rate,
            summary.avg_tokens_per_second,
            summary.min_tokens_per_second,
            summary.max_tokens_per_second,
            summary.avg_ttft_ms
        );
    }
}

pub fn print_results_markdown(summaries: &[ModelSummary], duration: Duration) {
    println!("# Benchmark Results\n");
    
    println!("| Model | Success Rate | Avg Speed | Min Speed | Max Speed | Avg TTFT |");
    println!("|-------|--------------|-----------|-----------|-----------|----------|");
    
    for summary in summaries {
        println!(
            "| {} | {:.1}% | {:.1} tok/s | {:.1} tok/s | {:.1} tok/s | {:.0}ms |",
            summary.model,
            summary.success_rate * 100.0,
            summary.avg_tokens_per_second,
            summary.min_tokens_per_second,
            summary.max_tokens_per_second,
            summary.avg_ttft_ms
        );
    }
    
    println!();
    
    if let Some(winner) = calculate_winner(summaries) {
        println!("## Winner: {} 🏆", winner.model);
        
        if summaries.len() > 1 {
            println!("\n### Performance Comparison:");
            for other in summaries {
                if other.model != winner.model && other.success_rate > 0.0 {
                    let (speed_diff, ttft_diff) = calculate_performance_difference(winner, other);
                    if speed_diff > 0.0 {
                        println!("- {:.1}% faster than {}", speed_diff, other.model);
                    }
                    if ttft_diff > 0.0 {
                        println!("- {:.0}% lower TTFT than {}", ttft_diff, other.model);
                    }
                }
            }
        }
    }
    
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    
    print!("\n*Total duration: ");
    if minutes > 0 {
        println!("{}m {}s*", minutes, seconds);
    } else {
        println!("{}s*", duration.as_secs());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_results_csv() {
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
        
        // This test just ensures the function doesn't panic
        print_results_csv(&summaries);
    }
}