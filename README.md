# Qubit Argument

[![Rust CI](https://github.com/qubit-ltd/rs-argument/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-argument/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-argument/coverage-badge.json)](https://qubit-ltd.github.io/rs-argument/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-argument.svg?color=blue)](https://crates.io/crates/qubit-argument)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Ownership-preserving argument validation for Rust.

## Overview

Qubit Argument provides extension traits and focused checking functions for
numeric values, durations, strings, collections, optional values, indexes, and
bounds.
Every validation failure is an `ArgumentError` with an owned argument path and
an inspectable `ArgumentErrorKind`. Validation APIs return `Result`; callers
choose whether to recover, convert the error, or explicitly treat a failure as
an internal invariant violation.

Successful validation returns the original owned value or borrow without
cloning it, so checks can be chained while preserving ownership.

Validation extension traits are sealed and implemented only for their
documented standard-library types. `ArgumentErrorKind`, `ArgumentValue`, and
`LengthMetric` are non-exhaustive; downstream matches must include a wildcard
arm so new structured cases can be added compatibly.

## Installation

The default feature set is empty (`default = []`), so core validation has no
runtime dependency on the regex engine.

```toml
[dependencies]
qubit-argument = "0.4"

# Enable only when regex validation is needed.
qubit-argument = { version = "0.4", features = ["regex"] }
```

## Quick Start

The traits and error types are exported directly from the crate root:

```rust
use qubit_argument::{ArgumentResult, NumericArgument, StringArgument};

fn validate_user(age: u8, name: String) -> ArgumentResult<(u8, String)> {
    let age = age.require_in_range("age", 0..=150)?;
    let name = name
        .require_non_blank("name")?
        .require_char_count_in("name", 3, 32)?;
    Ok((age, name))
}
```

Both `age` and `name` are returned in their original forms. In particular, the
owned `String` is not cloned during validation.

## Downstream Error Conversion

A downstream crate can preserve the structured error in its own domain error.
After implementing `From<ArgumentError>`, the `?` operator performs the
conversion directly:

```rust
use qubit_argument::{ArgumentError, NumericArgument};

#[derive(Debug)]
enum DomainError {
    InvalidArgument(ArgumentError),
}

impl From<ArgumentError> for DomainError {
    fn from(error: ArgumentError) -> Self {
        Self::InvalidArgument(error)
    }
}

fn validate_pool_size(size: u32) -> Result<u32, DomainError> {
    let size = size.require_positive("pool_size")?;
    Ok(size)
}
```

Optional values can use the same ownership-preserving validators without a
manual `if let`. `None` is left unchanged, while `Some` is validated and
rewrapped:

```rust
use qubit_argument::{ArgumentResult, NumericArgument, OptionArgument};

fn validate_stack_size(
    stack_size: Option<usize>,
) -> ArgumentResult<Option<usize>> {
    stack_size.validate_some(|value| {
        value.require_positive("stack_size")
    })
}
```

Use `ArgumentError::path()` and `ArgumentError::kind()` for programmatic
decisions. The `Display` text is intended for diagnostics, not parsing;
caller-provided paths, patterns, custom codes, and messages are escaped so the
diagnostic remains on one line without changing the structured values.

Nested validators can keep local field names and add parent context only when
they fail. `ArgumentResultExt::with_path_prefix` leaves `Ok` values unchanged
and turns a local path such as `connect` into `timeouts.connect` on `Err`.

```rust
use qubit_argument::{ArgumentResult, ArgumentResultExt, DurationArgument};
use std::time::Duration;

fn validate_timeouts(connect: Duration) -> ArgumentResult<Duration> {
    connect
        .require_positive("connect")
        .with_path_prefix("timeouts")
}
```

Domain-specific rules can retain a stable machine code and human-readable
message while still participating in the same structured error flow:

```rust
use qubit_argument::{ArgumentResult, require_that};

fn validate_retry_limit(limit: u32) -> ArgumentResult<u32> {
    require_that(
        limit,
        "retry_limit",
        |value| *value <= 10,
        "retry_limit",
        "retry limit must not exceed ten",
    )
}
```

Validation remains recoverable by default. When a value is an internal
invariant rather than external input, the caller may explicitly use `expect`
with a meaningful explanation:

```rust
use qubit_argument::NumericArgument;

fn built_in_retry_limit() -> u32 {
    3_u32
        .require_positive("retry_limit")
        .expect("the built-in retry limit is a positive internal invariant")
}
```

## Validation APIs

### Numeric values

`NumericArgument` supports all primitive integer types plus `f32` and `f64`:

- zero, non-zero, positive, non-negative, negative, and non-positive checks;
- `require_less_than`, `require_at_most`, `require_greater_than`, and
  `require_at_least` comparisons;
- `require_in_range` with standard inclusive, exclusive, and unbounded
  `RangeBounds`.

Floating-point values and range endpoints that are `NaN` are rejected. A
reversed or structurally empty range is reported separately from a value that
falls outside a valid range.

### Durations and finite floats

`DurationArgument` is implemented for `std::time::Duration`. It provides a
strictly positive check plus strict and inclusive comparisons. Structured
comparison errors retain the exact `Duration`, including its unit, rather than
converting it to an ambiguous integer.

`FloatArgument::require_finite` is implemented only for `f32` and `f64`. It
returns `NotANumber` for NaN and `NotFinite { actual }` for positive or negative
infinity. Finite values can continue into the comparison and range methods from
`NumericArgument`.

### Strings

`StringArgument` is implemented for `String` and `&str`:

- `require_non_blank` checks Unicode whitespace;
- `require_byte_len*` methods count UTF-8 bytes;
- `require_char_count*` methods count Unicode scalar values;
- with the `regex` feature, `require_match` and `require_not_match` accept a
  compiled `regex::Regex`.

Byte length and Unicode scalar count are deliberately distinct. For example,
`"汉😀"` contains seven UTF-8 bytes and two Unicode scalar values. Scalar
counts are not grapheme-cluster counts.

Regex validation uses `Regex::is_match` semantics and is not implicitly
anchored. Add `^` and `$` to the pattern when the entire string must match.

### Collections and optional values

`CollectionArgument` validates `Vec<T>`, `&[T]`, and `[T; N]` without cloning
elements. It provides non-empty, exact-length, minimum-length, maximum-length,
and inclusive length-range checks.

`OptionArgument::require_some` moves a present value out of its option.
`OptionArgument::validate_if_some` borrows a present value for validation,
skips the validator for `None`, and returns the original option without
cloning. `OptionArgument::validate_some` moves a present value through an
ownership-preserving validator, allowing validation or transformation without
requiring `Clone` or `Copy`; `None` is returned unchanged.

### Custom rules and bounds

- `require_that` applies a caller-provided predicate and reports a structured
  `Custom` error on failure.
- `check_bounds` validates an offset and length without unchecked addition.
- `check_element_index` and `check_position_index` distinguish element indexes
  from boundary positions.
- `check_position_range` validates a half-open position range.

## Error Privacy

String validation errors record the path, failure kind, observed length or
count, and any required constraint. They do not store the original validated
string, so `Debug` and `Display` do not reveal it. Pattern errors retain the
pattern, not the input. Callers of `require_that` remain responsible for
keeping their own custom code and message free of sensitive data.

`Length` and `InvalidLengthConstraint` errors also carry a `LengthMetric`:
`Bytes` for UTF-8 byte methods, `UnicodeScalars` for character-count methods,
and `Elements` for collection methods. Equal numeric lengths therefore remain
structurally distinguishable by measurement unit.

## Testing

```bash
# Core API with the default empty feature set
cargo test --no-default-features

# Core API plus regex validation
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-argument](https://github.com/qubit-ltd/rs-argument)
