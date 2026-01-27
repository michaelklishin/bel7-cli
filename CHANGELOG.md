# Change log

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
