use std::io::{self, Write};
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::config::PROGRESS_BAR_WIDTH;

pub trait ProgressReporter: Send {
    fn start_model(&mut self, model: &str, current: u32, total: u32);
    fn update_progress(&mut self, model: &str, current: u32, total: u32);
    fn complete_model(&mut self, model: &str);
    fn print_info(&mut self, message: &str);
    #[allow(dead_code)]
    fn print_error(&mut self, message: &str);
}

pub struct TerminalProgress {
    quiet: bool,
    #[allow(dead_code)]
    verbose: bool,
}

impl TerminalProgress {
    pub fn new(quiet: bool, verbose: bool) -> Self {
        Self { quiet, verbose }
    }
    
    fn print_progress_bar(&self, current: u32, total: u32, model: &str) {
        if self.quiet {
            return;
        }
        
        let percentage = if total > 0 {
            (current * 100) / total
        } else {
            0
        };
        
        let filled = if total > 0 {
            (PROGRESS_BAR_WIDTH * current as usize) / total as usize
        } else {
            0
        };
        
        let empty = PROGRESS_BAR_WIDTH.saturating_sub(filled);
        let bar = "█".repeat(filled) + &"░".repeat(empty);
        
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            Print(format!("Testing {}... ", model)),
            SetForegroundColor(Color::Cyan),
            Print(&bar),
            ResetColor,
            Print(format!(" {}% ({}/{})", percentage, current, total))
        ).ok();
        
        io::stdout().flush().ok();
    }
}

impl ProgressReporter for TerminalProgress {
    fn start_model(&mut self, model: &str, current: u32, total: u32) {
        if !self.quiet {
            if current == 1 {
                println!("\n⚡ Benchmarking {} model{} with {} iteration{} each",
                    total,
                    if total > 1 { "s" } else { "" },
                    crate::config::DEFAULT_ITERATIONS,
                    if crate::config::DEFAULT_ITERATIONS > 1 { "s" } else { "" }
                );
            }
            println!("\nTesting {} ({}/{})...", model, current, total);
        }
    }
    
    fn update_progress(&mut self, model: &str, current: u32, total: u32) {
        self.print_progress_bar(current, total, model);
    }
    
    fn complete_model(&mut self, model: &str) {
        if !self.quiet {
            execute!(
                io::stdout(),
                cursor::MoveToColumn(0),
                Clear(ClearType::CurrentLine),
                Print("Testing "),
                Print(model),
                Print("... "),
                SetForegroundColor(Color::Green),
                Print("✓ Complete"),
                ResetColor,
                Print("\n")
            ).ok();
        }
    }
    
    fn print_info(&mut self, message: &str) {
        if !self.quiet {
            println!("{}", message);
        }
    }
    
    fn print_error(&mut self, message: &str) {
        eprintln!("{}", message);
    }
}

pub struct QuietProgress;

impl ProgressReporter for QuietProgress {
    fn start_model(&mut self, _model: &str, _current: u32, _total: u32) {}
    fn update_progress(&mut self, _model: &str, _current: u32, _total: u32) {}
    fn complete_model(&mut self, _model: &str) {}
    fn print_info(&mut self, _message: &str) {}
    fn print_error(&mut self, message: &str) {
        eprintln!("{}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_progress_creation() {
        let progress = TerminalProgress::new(false, false);
        assert!(!progress.quiet);
        assert!(!progress.verbose);
        
        let quiet_progress = TerminalProgress::new(true, false);
        assert!(quiet_progress.quiet);
    }
    
    #[test]
    fn test_quiet_progress() {
        let mut progress = QuietProgress;
        // These should not panic
        progress.start_model("test", 1, 1);
        progress.update_progress("test", 1, 1);
        progress.complete_model("test");
        progress.print_info("info");
        progress.print_error("error");
    }
}