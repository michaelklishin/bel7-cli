# Change log

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
