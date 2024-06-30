# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-06-30

### Improvement

Optimize vector use with `Arc`.
Removal of useless `Caret` ast.

### Removed

Removal of Bar - the absolute value symbol - from `eval_f64` and `eval_i64` (too unstable).
Removal of Pow2 and Pow3 with the associated superscript.

### New features

Add full superscript support for powers (`eval_i64`).
Add Bar - as `OR` - and Ampersand - as `AND` - (`eval_i64`).
Add `avg` function to `eval_f64` and `eval_i64`.

## [0.1.1] - 2024-06-30

Some minor fixes and tests has been introduced.
Update the readme and introduce some missing functions.

## [0.1.0] - 2024-06-29

Initial release.
