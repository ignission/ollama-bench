pub const APP_NAME: &str = "ollama-bench";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &str = "⚡ Apache Bench-style Ollama LLM performance benchmarking";

pub const DEFAULT_OLLAMA_BASE_URL: &str = "http://localhost:11434";
pub const DEFAULT_ITERATIONS: u32 = 5;
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 120;
pub const DEFAULT_TEMPERATURE: f32 = 0.7;
pub const DEFAULT_MAX_TOKENS: i32 = 100;

pub const DEFAULT_PROMPT: &str = "Write a haiku about benchmarking language models.";

pub const PROGRESS_BAR_WIDTH: usize = 32;
#[allow(dead_code)]
pub const PROGRESS_REFRESH_RATE_MS: u64 = 100;

pub const TABLE_COLUMN_WIDTHS: TableWidths = TableWidths {
    model: 13,
    avg_speed: 13,
    ttft: 13,
    success_rate: 14,
};

#[allow(dead_code)]
pub struct TableWidths {
    pub model: usize,
    pub avg_speed: usize,
    pub ttft: usize,
    pub success_rate: usize,
}

#[allow(dead_code)]
pub const WINNER_THRESHOLD_PERCENT: f64 = 5.0;

#[allow(dead_code)]
pub const TERMINAL_COLORS: TerminalColors = TerminalColors {
    success: "\x1b[32m",   // Green
    error: "\x1b[31m",     // Red
    warning: "\x1b[33m",   // Yellow
    info: "\x1b[36m",      // Cyan
    reset: "\x1b[0m",      // Reset
    bold: "\x1b[1m",       // Bold
};

#[allow(dead_code)]
pub struct TerminalColors {
    pub success: &'static str,
    pub error: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub reset: &'static str,
    pub bold: &'static str,
}

pub fn get_user_agent() -> String {
    format!("{}/{}", APP_NAME, APP_VERSION)
}

#[allow(dead_code)]
pub fn get_default_headers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Accept", "application/json"),
        ("Content-Type", "application/json"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_ITERATIONS, 5);
        assert_eq!(DEFAULT_TIMEOUT_SECONDS, 120);
        assert_eq!(DEFAULT_OLLAMA_BASE_URL, "http://localhost:11434");
    }
    
    #[test]
    fn test_user_agent() {
        let ua = get_user_agent();
        assert!(ua.starts_with(APP_NAME));
        assert!(ua.contains('/'));
    }
    
    #[test]
    fn test_default_headers() {
        let headers = get_default_headers();
        assert_eq!(headers.len(), 2);
        assert!(headers.iter().any(|(k, _)| *k == "Accept"));
        assert!(headers.iter().any(|(k, _)| *k == "Content-Type"));
    }
}