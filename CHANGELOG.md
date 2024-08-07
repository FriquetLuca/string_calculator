# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2024-07-22

### Improvement

The first character of a superscript won't randomly panic anymore.
Fix readme and update it for `eval_complex` and `eval_number`.

### New features

Introduction of `eval_complex` and `eval_number`.


## [0.3.3] - 2024-07-03

### Improvement

Remove `conv` dependency.
`rust_decimal` is now an optional dependency used only for the feature `eval_decimal`.

### New features

Allow the use of features to compile specific eval (named after the corresponding function).

## [0.3.2] - 2024-07-02

### Removed

Remove `statrs` dependency.

### New features

Custom implementation of gamma function.
Implement all the missing functions (except for the trigonometric ones) of `eval_decimal`.
Add `lb` - binary lobarithm -.
Add `gcd` and `lcm` to `eval_i64`.
Add `w` - lambert w - and `ilog` - iterated logarithm - to `eval_f64` and `eval_decimal`.

## [0.3.1] - 2024-07-02

### Improvement

Update `README` in accordance to all missing functions added in `eval_decimal` and the `root` function argument definition has been also corrected.
Patch the parsing of numbers in the format `.XXXX` for decimals.

### Removed

Remove `statrs` dependency.

### New features

Custom implementation of gamma function.
Implement all the missing functions (except for the trigonometric ones) of `eval_decimal`.
Add `lb` - binary lobarithm -.
Add `gcd` and `lcm` to `eval_i64`.
Add `w` - lambert w - and `ilog` - iterated logarithm - to `eval_f64` and `eval_decimal`.

## [0.3.0] - 2024-07-01

Add `eval_decimal`.

## [0.2.2] - 2024-06-30

Isolation of superscript deserialization.

## [0.2.1] - 2024-06-30

Small refactor of operator's category and parser error.

## [0.2.0] - 2024-06-30

### Improvement

Optimize vector use with `Arc`.
Removal of useless `Caret` ast.
Less string extract in tokenizer for comparison, making the tokenizer faster.

### Removed

Removal of Bar - the absolute value symbol - from `eval_f64` and `eval_i64` (too unstable).
Removal of Pow2 and Pow3 with the associated superscript.

### New features

Add full superscript support for powers (`eval_i64`).
Add Bar - as `OR` - and Ampersand - as `AND` - (`eval_i64`).
Add `avg` and `median` function to `eval_f64` and `eval_i64`.

## [0.1.1] - 2024-06-30

Some minor fixes and tests has been introduced.
Update the readme and introduce some missing functions.

## [0.1.0] - 2024-06-29

Initial release.
