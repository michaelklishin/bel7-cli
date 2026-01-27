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

#![cfg(feature = "progress")]

use std::time::Duration;

use bel7_cli::{
    BRAILLE_TICK_CHARS, DownloadReporter, InteractiveReporter, NonInteractiveReporter,
    ProgressReporter, QuietReporter, SpinnerReporter, select_reporter,
};

#[test]
fn test_interactive_reporter_lifecycle() {
    let mut reporter = InteractiveReporter::new();
    reporter.start(3, "Processing");
    reporter.progress(0, 3, "item1");
    reporter.success("item1");
    reporter.progress(1, 3, "item2");
    reporter.skip("item2", "already done");
    reporter.progress(2, 3, "item3");
    reporter.failure("item3", "some error");
    reporter.finish(3);
}

#[test]
fn test_non_interactive_reporter_lifecycle() {
    let mut reporter = NonInteractiveReporter::new();
    reporter.start(2, "Building");
    reporter.progress(0, 2, "step1");
    reporter.success("step1");
    reporter.progress(1, 2, "step2");
    reporter.failure("step2", "failed");
    reporter.finish(2);
}

#[test]
fn test_quiet_reporter_lifecycle() {
    let mut reporter = QuietReporter::new();
    reporter.start(5, "Testing");
    reporter.progress(0, 5, "test1");
    reporter.success("test1");
    reporter.skip("test2", "disabled");
    reporter.failure("test3", "assertion failed");
    reporter.finish(5);
}

#[test]
fn test_select_reporter_quiet() {
    let reporter = select_reporter(true, false);
    let _ = reporter;
}

#[test]
fn test_select_reporter_non_interactive() {
    let reporter = select_reporter(false, true);
    let _ = reporter;
}

#[test]
fn test_select_reporter_interactive() {
    let reporter = select_reporter(false, false);
    let _ = reporter;
}

#[test]
fn test_select_reporter_quiet_takes_precedence() {
    let reporter = select_reporter(true, true);
    let _ = reporter;
}

#[test]
fn test_interactive_reporter_default() {
    let reporter = InteractiveReporter::default();
    let _ = reporter;
}

#[test]
fn test_non_interactive_reporter_default() {
    let reporter = NonInteractiveReporter::default();
    let _ = reporter;
}

#[test]
fn test_quiet_reporter_default() {
    let reporter = QuietReporter::default();
    let _ = reporter;
}

#[test]
fn test_spinner_reporter_lifecycle() {
    let mut spinner = SpinnerReporter::new();
    spinner.start("Loading...");
    spinner.set_message("Still loading...");
    spinner.finish("Done!");
}

#[test]
fn test_spinner_reporter_finish_and_clear() {
    let mut spinner = SpinnerReporter::new();
    spinner.start("Working...");
    spinner.finish_and_clear();
}

#[test]
fn test_spinner_reporter_default() {
    let spinner = SpinnerReporter::default();
    let _ = spinner;
}

#[test]
fn test_spinner_set_message_without_start() {
    let spinner = SpinnerReporter::new();
    spinner.set_message("test");
}

#[test]
fn test_spinner_finish_without_start() {
    let mut spinner = SpinnerReporter::new();
    spinner.finish("done");
}

#[test]
fn test_interactive_reporter_all_failures() {
    let mut reporter = InteractiveReporter::new();
    reporter.start(2, "Failing");
    reporter.progress(0, 2, "item1");
    reporter.failure("item1", "error1");
    reporter.progress(1, 2, "item2");
    reporter.failure("item2", "error2");
    reporter.finish(2);
}

#[test]
fn test_interactive_reporter_all_successes() {
    let mut reporter = InteractiveReporter::new();
    reporter.start(2, "Succeeding");
    reporter.progress(0, 2, "item1");
    reporter.success("item1");
    reporter.progress(1, 2, "item2");
    reporter.success("item2");
    reporter.finish(2);
}

#[test]
fn test_interactive_reporter_finish_without_bar() {
    let mut reporter = InteractiveReporter::new();
    reporter.finish(0);
}

#[test]
fn test_non_interactive_reporter_finish_without_bar() {
    let mut reporter = NonInteractiveReporter::new();
    reporter.finish(0);
}

#[test]
fn test_progress_reporter_trait_object() {
    fn use_reporter(reporter: &mut dyn ProgressReporter) {
        reporter.start(1, "test");
        reporter.progress(0, 1, "item");
        reporter.success("item");
        reporter.finish(1);
    }

    use_reporter(&mut InteractiveReporter::new());
    use_reporter(&mut NonInteractiveReporter::new());
    use_reporter(&mut QuietReporter::new());
}

#[test]
fn test_boxed_reporter() {
    let mut reporter: Box<dyn ProgressReporter> = Box::new(QuietReporter::new());
    reporter.start(1, "boxed");
    reporter.finish(1);
}

#[test]
fn test_quiet_reporter_clone() {
    let reporter = QuietReporter::new();
    let cloned = reporter.clone();
    let _ = cloned;
}

#[test]
fn test_quiet_reporter_copy() {
    let reporter = QuietReporter::new();
    let copied = reporter;
    let _ = copied;
}

#[test]
fn test_interactive_reporter_debug() {
    let reporter = InteractiveReporter::new();
    let debug = format!("{:?}", reporter);
    assert!(debug.contains("InteractiveReporter"));
}

#[test]
fn test_non_interactive_reporter_debug() {
    let reporter = NonInteractiveReporter::new();
    let debug = format!("{:?}", reporter);
    assert!(debug.contains("NonInteractiveReporter"));
}

#[test]
fn test_spinner_reporter_debug() {
    let reporter = SpinnerReporter::new();
    let debug = format!("{:?}", reporter);
    assert!(debug.contains("SpinnerReporter"));
}

#[test]
fn test_quiet_reporter_debug() {
    let reporter = QuietReporter::new();
    let debug = format!("{:?}", reporter);
    assert!(debug.contains("QuietReporter"));
}

#[test]
fn test_spinner_with_custom_tick_chars() {
    let mut spinner = SpinnerReporter::new().with_tick_chars(BRAILLE_TICK_CHARS);
    spinner.start("Custom spinner...");
    spinner.finish_and_clear();
}

#[test]
fn test_spinner_with_custom_tick_interval() {
    let mut spinner = SpinnerReporter::new().with_tick_interval(Duration::from_millis(50));
    spinner.start("Fast spinner...");
    spinner.finish_and_clear();
}

#[test]
fn test_spinner_with_both_customizations() {
    let mut spinner = SpinnerReporter::new()
        .with_tick_chars(&["⠋", "⠙", "⠹", "⠸"])
        .with_tick_interval(Duration::from_millis(80));
    spinner.start("Customized...");
    spinner.set_message("Still customized...");
    spinner.finish("Done!");
}

#[test]
fn test_braille_tick_chars_constant() {
    assert_eq!(BRAILLE_TICK_CHARS.len(), 10);
    assert!(BRAILLE_TICK_CHARS.contains(&"⠋"));
}

#[test]
fn test_download_reporter_lifecycle() {
    let mut reporter = DownloadReporter::new();
    reporter.start(1024 * 1024, "Downloading file.tar.gz");
    reporter.add_bytes(1024);
    reporter.add_bytes(2048);
    reporter.set_message("Still downloading...");
    reporter.finish("Download complete");
}

#[test]
fn test_download_reporter_set_position() {
    let mut reporter = DownloadReporter::new();
    reporter.start(1000, "Downloading");
    reporter.set_position(500);
    reporter.set_position(750);
    reporter.finish_and_clear();
}

#[test]
fn test_download_reporter_finish_and_clear() {
    let mut reporter = DownloadReporter::new();
    reporter.start(100, "Test");
    reporter.add_bytes(50);
    reporter.finish_and_clear();
}

#[test]
fn test_download_reporter_default() {
    let reporter = DownloadReporter::default();
    let _ = reporter;
}

#[test]
fn test_download_reporter_debug() {
    let reporter = DownloadReporter::new();
    let debug = format!("{:?}", reporter);
    assert!(debug.contains("DownloadReporter"));
}

#[test]
fn test_download_reporter_without_start() {
    let reporter = DownloadReporter::new();
    reporter.add_bytes(100);
    reporter.set_position(50);
    reporter.set_message("test");
}

#[test]
fn test_download_reporter_finish_without_start() {
    let mut reporter = DownloadReporter::new();
    reporter.finish("done");
}
