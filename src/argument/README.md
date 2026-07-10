# Argument Validation Guide

Author: Haixing Hu

## Purpose

Qubit Argument validates values at API and configuration boundaries without
changing their ownership shape. Public items are imported directly from
`qubit_argument`; the implementation modules are private.

Every validation function returns `ArgumentResult<T>`, an alias for
`Result<T, ArgumentError>`. An `ArgumentError` contains an `ArgumentPath` and a
non-exhaustive `ArgumentErrorKind`, allowing downstream code to match failures
without parsing diagnostic text.

## Error Model and Constraint Types

All error-model types are also exported from the crate root:

| API | Purpose |
| --- | --- |
| `ArgumentError::new`, `path`, `kind`, `into_parts` | Construct or inspect an owned structured failure |
| `ArgumentPath::new`, `as_str` | Store and borrow an argument or nested-field path |
| `ArgumentErrorKind` | Match `Missing`, `Blank`, `Empty`, `Length`, `Comparison`, `Range`, invalid-constraint, NaN, index, bounds, pattern, and custom failures |
| `ArgumentValue` | Preserve signed, unsigned, `f32`, or `f64` values without numeric loss |
| `LengthConstraint`, `ComparisonConstraint` | Describe length and numeric-comparison requirements |
| `ArgumentBound`, `RangeConstraint::new`, `lower`, `upper`, `into_bounds` | Describe inclusive, exclusive, or unbounded numeric ranges |
| `IndexRole`, `PatternExpectation` | Distinguish index domains and regex match expectations |

`ArgumentError` owns this context and implements `std::error::Error`. Its
`Display` output is diagnostic text rather than a stable parsing protocol.

## Ownership-Preserving Chains

Extension-trait methods consume and return `Self`. Owned values remain owned,
borrowed values remain borrowed, and successful validation does not clone the
input.

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

## Structured Errors and Downstream Conversion

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

fn validate_workers(workers: usize) -> Result<usize, DomainError> {
    let workers = workers.require_positive("workers")?;
    Ok(workers)
}
```

The conversion preserves the complete structured error and lets `?` propagate
it naturally. Inspect `error.path()` and `error.kind()` for program logic;
reserve `Display` for human-readable diagnostics.

For a value that is guaranteed by program construction, the caller may
explicitly escalate a validation error with a documented `expect`:

```rust
use qubit_argument::NumericArgument;

let workers = 4_usize
    .require_positive("workers")
    .expect("the compiled-in worker count is a positive internal invariant");
assert_eq!(workers, 4);
```

## Numeric Validation

`NumericArgument` supports all primitive integer types, `f32`, and `f64`.
Successful methods return the original numeric value without cloning.

| Methods | Failure kinds |
| --- | --- |
| `require_zero`, `require_non_zero`, `require_positive`, `require_non_negative`, `require_negative`, `require_non_positive` | `Comparison`; `NotANumber` for NaN |
| `require_less_than`, `require_at_most`, `require_greater_than`, `require_at_least` | `Comparison`; `NotANumber` when the value or bound is NaN |
| `require_in_range` | `Range`, `InvalidRangeConstraint`, or `NotANumber` |

`require_in_range` accepts standard `RangeBounds`, including inclusive,
exclusive, and unbounded endpoints. Reversed bounds and equal endpoints with an
excluded side produce `InvalidRangeConstraint`.

## String Validation

`StringArgument` is implemented for `String` and `&str`. Every successful
method returns the original string value or borrow without cloning, and no
string-validation error stores the original inspected string.

| Methods | Measurement | Failure kinds |
| --- | --- | --- |
| `require_non_blank` | Unicode whitespace | `Blank` |
| `require_byte_len`, `require_byte_len_at_least`, `require_byte_len_at_most` | UTF-8 bytes | `Length` |
| `require_byte_len_in` | UTF-8 bytes | `Length` or `InvalidLengthConstraint` |
| `require_char_count`, `require_char_count_at_least`, `require_char_count_at_most` | Unicode scalar values | `Length` |
| `require_char_count_in` | Unicode scalar values | `Length` or `InvalidLengthConstraint` |
| `require_match`, `require_not_match` (`regex` feature) | `Regex::is_match` | `Pattern` |

Byte length and Unicode scalar count are not interchangeable. `"汉😀"` has a
byte length of seven and a scalar count of two. Scalar values are also not
grapheme clusters.

The regex methods are available only with the optional `regex` feature. They
use `Regex::is_match` directly, so matching is not implicitly anchored. Use
anchors in the pattern when whole-string matching is required.

## Collection Validation

`CollectionArgument` is implemented for `Vec<T>`, `&[T]`, and `[T; N]`.
Successful validation returns the original collection without cloning its
elements.

| Method | Failure kinds |
| --- | --- |
| `require_non_empty` | `Empty` |
| `require_len`, `require_len_at_least`, `require_len_at_most` | `Length` |
| `require_len_in` | `Length` or `InvalidLengthConstraint` |

## Option Validation

`OptionArgument::require_some` moves a present value out without cloning and
returns `Missing` for `None`. `validate_if_some` borrows a present value for a
caller-supplied validator and returns the original option without cloning. It
does not call the validator for `None` and propagates the validator's
`ArgumentError` unchanged.

## Custom and Bounds Validation

| Function | Success value | Failure kind |
| --- | --- | --- |
| `require_that` | Original value, without cloning | `Custom` |
| `check_bounds` | `()` | `Bounds` |
| `check_element_index` | Original index | `Index` with `IndexRole::Element` |
| `check_position_index` | Original index | `Index` with `IndexRole::Position` |
| `check_position_range` | Validated `start..end` | `IndexRange` |

`check_bounds` compares before subtracting, avoiding unchecked
`offset + length` arithmetic. `require_that` copies the caller-provided path,
code, and message only on failure; callers must keep custom messages free of
sensitive data.

## Features

The default feature set is empty. Enable regex validation only when needed:

```toml
[dependencies]
qubit-argument = "0.4"

# Enable only when regex validation is needed.
qubit-argument = { version = "0.4", features = ["regex"] }
```

## Verification

```bash
cargo test --doc --no-default-features
cargo test --doc --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features
```
