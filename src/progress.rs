// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Progress reporting utilities for CLI operations.

use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

/// A trait for reporting progress during multi-item operations.
pub trait ProgressReporter {
    /// Called when starting a batch operation.
    fn start(&mut self, total: usize, operation_name: &str);

    /// Called to report progress on an individual item.
    fn progress(&mut self, current: usize, total: usize, item_name: &str);

    /// Called when an item succeeds.
    fn success(&mut self, item_name: &str);

    /// Called when an item is skipped.
    fn skip(&mut self, item_name: &str, reason: &str);

    /// Called when an item fails.
    fn failure(&mut self, item_name: &str, error: &str);

    /// Called when the batch operation finishes.
    fn finish(&mut self, total: usize);
}

/// Progress reporter with an interactive progress bar.
#[derive(Debug)]
pub struct InteractiveReporter {
    bar: Option<ProgressBar>,
    failures: usize,
}

impl InteractiveReporter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bar: None,
            failures: 0,
        }
    }
}

impl Default for InteractiveReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressReporter for InteractiveReporter {
    fn start(&mut self, total: usize, operation_name: &str) {
        let bar = ProgressBar::new(total as u64);
        let style = ProgressStyle::with_template(
            "{msg} [{bar:40.green/red}] {pos}/{len} ({percent}%) {elapsed_precise}",
        )
        .unwrap_or_else(|_| ProgressStyle::default_bar());
        bar.set_style(style);
        bar.set_message(operation_name.to_string());
        self.bar = Some(bar);
        self.failures = 0;
    }

    fn progress(&mut self, _current: usize, _total: usize, _item_name: &str) {
        if let Some(bar) = &self.bar {
            bar.inc(1);
        }
    }

    fn success(&mut self, _item_name: &str) {}

    fn skip(&mut self, _item_name: &str, _reason: &str) {}

    fn failure(&mut self, _item_name: &str, _error: &str) {
        self.failures += 1;
    }

    fn finish(&mut self, total: usize) {
        if let Some(bar) = self.bar.take() {
            bar.finish();
            let successes = total.saturating_sub(self.failures);
            if self.failures == 0 {
                println!("Completed: {} items processed successfully", total);
            } else if successes == 0 {
                println!("Failed: all {} items failed", total);
            } else {
                println!(
                    "Completed with failures: {} succeeded, {} failed of {} total",
                    successes, self.failures, total
                );
            }
        }
    }
}

/// Progress reporter for non-interactive environments.
#[derive(Debug)]
pub struct NonInteractiveReporter {
    bar: Option<ProgressBar>,
}

impl NonInteractiveReporter {
    #[must_use]
    pub fn new() -> Self {
        Self { bar: None }
    }
}

impl Default for NonInteractiveReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressReporter for NonInteractiveReporter {
    fn start(&mut self, total: usize, operation_name: &str) {
        let bar = ProgressBar::new(total as u64);
        let style = ProgressStyle::with_template("{msg}: {pos}/{len} [{elapsed_precise}]")
            .unwrap_or_else(|_| ProgressStyle::default_bar());
        bar.set_style(style);
        bar.set_message(operation_name.to_string());
        self.bar = Some(bar);
    }

    fn progress(&mut self, _current: usize, _total: usize, _item_name: &str) {
        if let Some(bar) = &self.bar {
            bar.inc(1);
        }
    }

    fn success(&mut self, _item_name: &str) {}

    fn skip(&mut self, _item_name: &str, _reason: &str) {}

    fn failure(&mut self, _item_name: &str, _error: &str) {}

    fn finish(&mut self, total: usize) {
        if let Some(bar) = self.bar.take() {
            bar.finish();
            println!("Completed: {} items processed", total);
        }
    }
}

/// Progress reporter that produces no output.
#[derive(Debug, Clone, Copy, Default)]
pub struct QuietReporter;

impl QuietReporter {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl ProgressReporter for QuietReporter {
    fn start(&mut self, _total: usize, _operation_name: &str) {}
    fn progress(&mut self, _current: usize, _total: usize, _item_name: &str) {}
    fn success(&mut self, _item_name: &str) {}
    fn skip(&mut self, _item_name: &str, _reason: &str) {}
    fn failure(&mut self, _item_name: &str, _error: &str) {}
    fn finish(&mut self, _total: usize) {}
}

/// Selects a progress reporter based on mode flags.
#[must_use]
pub fn select_reporter(quiet: bool, non_interactive: bool) -> Box<dyn ProgressReporter> {
    match (quiet, non_interactive) {
        (true, _) => Box::new(QuietReporter::new()),
        (false, true) => Box::new(NonInteractiveReporter::new()),
        (false, false) => Box::new(InteractiveReporter::new()),
    }
}

/// Default Braille spinner characters.
pub const BRAILLE_TICK_CHARS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Progress reporter using a spinner for indeterminate operations.
#[derive(Debug)]
pub struct SpinnerReporter {
    bar: Option<ProgressBar>,
    tick_chars: Option<Vec<String>>,
    tick_interval: Duration,
}

impl SpinnerReporter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bar: None,
            tick_chars: None,
            tick_interval: Duration::from_millis(100),
        }
    }

    /// Sets custom tick characters for the spinner animation.
    #[must_use]
    pub fn with_tick_chars(mut self, chars: &[&str]) -> Self {
        self.tick_chars = Some(chars.iter().map(|s| (*s).to_string()).collect());
        self
    }

    /// Sets the tick interval for spinner animation.
    #[must_use]
    pub fn with_tick_interval(mut self, interval: Duration) -> Self {
        self.tick_interval = interval;
        self
    }

    /// Starts the spinner with a message.
    pub fn start(&mut self, message: &str) {
        let bar = ProgressBar::new_spinner();

        let style = if let Some(ref chars) = self.tick_chars {
            let tick_strs: Vec<&str> = chars.iter().map(|s| s.as_str()).collect();
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap_or_else(|_| ProgressStyle::default_spinner())
                .tick_strings(&tick_strs)
        } else {
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap_or_else(|_| ProgressStyle::default_spinner())
        };

        bar.set_style(style);
        bar.set_message(message.to_string());
        bar.enable_steady_tick(self.tick_interval);
        self.bar = Some(bar);
    }

    /// Updates the spinner message.
    pub fn set_message(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.set_message(message.to_string());
        }
    }

    /// Finishes the spinner with a final message.
    pub fn finish(&mut self, message: &str) {
        if let Some(bar) = self.bar.take() {
            bar.finish_with_message(message.to_string());
        }
    }

    /// Finishes and clears the spinner.
    pub fn finish_and_clear(&mut self) {
        if let Some(bar) = self.bar.take() {
            bar.finish_and_clear();
        }
    }
}

impl Default for SpinnerReporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress reporter for download operations showing bytes and speed.
#[derive(Debug)]
pub struct DownloadReporter {
    bar: Option<ProgressBar>,
}

impl DownloadReporter {
    #[must_use]
    pub fn new() -> Self {
        Self { bar: None }
    }

    /// Starts the download progress bar with total size in bytes.
    pub fn start(&mut self, total_bytes: u64, message: &str) {
        let bar = ProgressBar::new(total_bytes);
        let style = ProgressStyle::with_template(
            "{msg} [{bar:40.cyan}] {bytes}/{total_bytes} ({bytes_per_sec}) {elapsed_precise}",
        )
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("█▉▊▋▌▍▎▏  ");
        bar.set_style(style);
        bar.set_message(message.to_string());
        self.bar = Some(bar);
    }

    /// Updates progress by adding downloaded bytes.
    pub fn add_bytes(&self, bytes: u64) {
        if let Some(bar) = &self.bar {
            bar.inc(bytes);
        }
    }

    /// Sets the current position in bytes.
    pub fn set_position(&self, bytes: u64) {
        if let Some(bar) = &self.bar {
            bar.set_position(bytes);
        }
    }

    /// Updates the message.
    pub fn set_message(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.set_message(message.to_string());
        }
    }

    /// Finishes the download with a final message.
    pub fn finish(&mut self, message: &str) {
        if let Some(bar) = self.bar.take() {
            bar.finish_with_message(message.to_string());
        }
    }

    /// Finishes and clears the progress bar.
    pub fn finish_and_clear(&mut self) {
        if let Some(bar) = self.bar.take() {
            bar.finish_and_clear();
        }
    }
}

impl Default for DownloadReporter {
    fn default() -> Self {
        Self::new()
    }
}
