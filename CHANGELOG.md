# Change log

## 0.13.0 (in development)

### Dependencies

 * `tabled` upgraded to `0.21.0`


## 0.12.0 (May 29, 2026)

### Enhancements

  * New exit code `3` (`PARTIAL_SUCCESS_I32`, also re-exported as `codes::PARTIAL_SUCCESS`) for commands that ran but didn't finish every unit of work
  * New `PARTIAL_SUCCESS_U8: u8 = 3` (also re-exported as `codes::PARTIAL_SUCCESS_U8`) for consumers that pipe through `process::ExitCode::from(u8)` without the `as u8` cast; a compile-time assertion pins the u8-fit so the constant can't silently move out of range
  * New `Outcome<E: ExitCodeProvider>` enum (`Success`, `PartialSuccess`, `Failure(E)`) — lets a command report "some of it landed" without faking an error
  * `Outcome` provides `exit_code_i32`, `into_result_u8`, `is_success`, `is_partial_success`, `is_failure`, and a `Display` impl
  * New `run_with_outcome`: like `run_with_exit_code` but takes an `Outcome<E>` instead of a `Result<(), E>`
  * New `ExitOutcome` enum (`Success`, `PartialSuccess`, `Failure(i32)`) for the other direction: reading another process's exit code from Rust
  * On non-Windows, `ExitOutcome::from_status` encodes signal kills as `Failure(128 + signum)` — the shell convention, so `SIGTERM` becomes `143`, `SIGINT` becomes `130`, `SIGKILL` becomes `137` — so callers can tell signals apart
  * `ExitOutcome` provides `is_success`, `is_partial_success`, `is_failure`, `from_code`, `from_status`, `is_success_shaped`, `code`, `canonicalize`, and a `Display` impl
  * `ExitOutcome::canonicalize` collapses a manually-built `Failure(0)` to `Success` and `Failure(3)` to `PartialSuccess`, useful when an `ExitOutcome` is built directly from a JSON envelope's integer field

### Dependency Upgrades

  * Upgrade `owo-colors` to `4.3.0`
  * Upgrade `thiserror` to `2.0.18`
  * Upgrade `clap` to `4.6.0`
  * Upgrade `clap_complete` to `4.6.2`
  * Upgrade `clap_complete_nushell` to `4.6.0`
  * Upgrade `indicatif` to `0.18.4`
  * Upgrade `terminal_size` to `0.4.4`
  * Upgrade `proptest` (dev) to `1.11.0`

## 0.9.0(Feb 22, 2026)

### Dependency Upgrades

 * Upgrade `sysexits` to `0.11.0`

## 0.8.0 (Jan 27, 2026)

### Enhancements

  * Test helpers for resetting shell-specific environment variables in tests

## 0.7.0 (Jan 27, 2026)

### Enhancements

  * New `completions` feature: shell completion generation for Bash, Zsh, Fish, Elvish, Nushell, and PowerShell
  * New `progress` feature: progress reporting utilities with `ProgressReporter` trait and several implementations extracted from `rabbitmqadmin` v2, `rabbitmq-lqt`, `frm`
  * New `DownloadReporter` for download progress with bytes and download speed display (à la `curl`, `wget`)
  * `SpinnerReporter` now supports customizable tick characters via `with_tick_chars()` and tick interval via `with_tick_interval`
  * New table helpers: `parse_columns` and `build_table_with_columns` for column filtering
  * More table helpers: `terminal_width`, `responsive_width`, and `DEFAULT_TERMINAL_WIDTH` for responsive layouts
  * `StyledTable` now supports `max_width` and `wrap_column` for responsive table rendering
  * Output functions such as `print_success`, `print_error` now respect `NO_COLOR` environment variable and detect non-TTY output
  * New functions: `should_colorize` and `should_colorize_stderr` for manual color control

## 0.6.0 (Jan 18, 2026)

### Enhancements

  * Removed the slow to compile (via its `proc-macro` transient dep) `clap/derive` dependency

## 0.5.0 (Dec 22, 2025)

### Bug Fixes

  * `StyledTable::build` now removes column headers before adding a panel one

### Enhancements

  * `TableStyle`: `serde` support (behind a feature)
  * `TableStyle`: `clap` support (behind a feature)

## 0.4.0 (Dec 22, 2025)

### Enhancements

  * New utility functions: `StyledTable#remove_header_row`, `StyledTable#padding`, `StyledTable#replace_newlines`

## 0.3.0 (Dec 22, 2025)

### Enhancements

  * Initial release
