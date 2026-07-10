# rs-argument v0.4 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the message-only validation API with ownership-preserving validators that return a structured `ArgumentError` suitable for automatic downstream `From` conversion.

**Architecture:** Keep `src/argument/` as a private implementation module. Model failures as `ArgumentError { path, kind }`, use public value and constraint types for stable matching, and implement type-specific extension traits over that shared error model. Keep the core dependency-free and expose regex validation only through an optional feature.

**Tech Stack:** Rust 1.94, edition 2024, standard library, optional `regex` 1.12, integration tests under `tests/`, shared `.rs-ci` scripts.

## Global Constraints

- Work only in `/home/starfish/working/qubit/rust-common/rs-argument`.
- Do not modify any real downstream `rs-*` repository.
- Breaking changes are required; do not add final compatibility aliases or deprecated wrappers.
- Public string parameters use `&str`; allocations occur only while constructing an error or preserving owned input.
- Every source function and method, including private helpers, has an English Rust doc comment covering inputs, outputs, errors, and relevant constraints.
- Every concrete source file imports its own dependencies explicitly; do not use `use super::*`.
- Tests remain outside `src/`, mirror the source layout, and use `test_{method}_{scenario}` names.
- Do not use unchecked `offset + length` arithmetic in bounds validation.
- Do not expose raw validated string contents through `ArgumentError`, `Debug`, or `Display`.
- The final default feature set is empty; `regex` is optional and disabled by default.
- Do not run `git add`, `git commit`, or `git push`; the user has not authorized repository writes beyond working-tree files.
- Use `git --no-pager diff --check` as the review checkpoint after each task.
- Final verification order is exactly `./align-ci.sh` followed by `./ci-check.sh`.

---

## File Map

**Create:**

- `src/argument/argument_path.rs` — owned argument-path newtype.
- `src/argument/argument_value.rs` — equality-safe representation of primitive numeric values.
- `src/argument/constraint.rs` — public length, comparison, range, index, and pattern constraint types.
- `tests/argument/mod.rs` — test-module index required by the repository test-layout rules.
- `tests/argument/argument_path_tests.rs` — path behavior.
- `tests/argument/argument_value_tests.rs` — numeric representation behavior.
- `tests/argument/constraint_tests.rs` — constraint construction and equality.
- `tests/argument/domain_error_conversion_tests.rs` — simulated downstream `From` conversion.

**Rename with `command mv`:**

- `src/argument/condition.rs` → `src/argument/bounds.rs`.
- `tests/argument/condition_tests.rs` → `tests/argument/bounds_tests.rs`.

**Rewrite or modify:**

- `src/argument/argument_error.rs`
- `src/argument/numeric_argument.rs`
- `src/argument/string_argument.rs`
- `src/argument/collection_argument.rs`
- `src/argument/option_argument.rs`
- `src/argument/mod.rs`
- `src/lib.rs`
- `tests/argument/argument_error_tests.rs`
- `tests/argument/numeric_argument_tests.rs`
- `tests/argument/string_argument_tests.rs`
- `tests/argument/collection_argument_tests.rs`
- `tests/argument/option_argument_tests.rs`
- `tests/argument_tests.rs`
- `Cargo.toml`
- `Cargo.lock`
- `README.md`
- `README.zh_CN.md`
- `src/argument/README.md`
- `src/argument/README.zh_CN.md`

---

### Task 1: Add the structured path, value, and constraint vocabulary

**Files:**

- Create: `src/argument/argument_path.rs`
- Create: `src/argument/argument_value.rs`
- Create: `src/argument/constraint.rs`
- Create: `tests/argument/argument_path_tests.rs`
- Create: `tests/argument/argument_value_tests.rs`
- Create: `tests/argument/constraint_tests.rs`
- Create: `tests/argument/mod.rs`
- Modify: `src/argument/mod.rs`
- Modify: `src/lib.rs`
- Modify: `tests/argument_tests.rs`

**Interfaces:**

- Produces `ArgumentPath`, `ArgumentValue`, `LengthConstraint`, `ComparisonConstraint`, `ArgumentBound`, `RangeConstraint`, `IndexRole`, and `PatternExpectation`.
- Later tasks consume these types from `crate::argument` internally and from `qubit_argument` in integration tests.

- [ ] **Step 1: Normalize the external test module layout**

Replace the inline module body in `tests/argument_tests.rs` with:

```rust
// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Integration tests for argument validation.

mod argument;
```

Create `tests/argument/mod.rs` with the existing modules plus the new vocabulary tests:

```rust
pub(crate) mod argument_error_tests;
pub(crate) mod argument_path_tests;
pub(crate) mod argument_value_tests;
pub(crate) mod collection_argument_tests;
pub(crate) mod condition_tests;
pub(crate) mod constraint_tests;
pub(crate) mod domain_error_conversion_tests;
pub(crate) mod numeric_argument_tests;
pub(crate) mod option_argument_tests;
pub(crate) mod string_argument_tests;
```

Create `tests/argument/domain_error_conversion_tests.rs` as an empty, documented test module for Task 2:

```rust
//! Simulated downstream error-conversion tests.
```

- [ ] **Step 2: Write failing vocabulary tests**

Create focused tests containing these assertions:

```rust
#[test]
fn test_argument_path_exposes_owned_path() {
    let path = ArgumentPath::new("retry.max_attempts");
    assert_eq!(path.as_str(), "retry.max_attempts");
    assert_eq!(path.as_ref(), "retry.max_attempts");
    assert_eq!(path.to_string(), "retry.max_attempts");
}

#[test]
fn test_argument_value_preserves_float_bits() {
    let negative_zero = ArgumentValue::from(-0.0_f64);
    let positive_zero = ArgumentValue::from(0.0_f64);
    assert_ne!(negative_zero, positive_zero);
    assert_eq!(negative_zero.to_string(), "-0");
}

#[test]
fn test_range_constraint_preserves_bound_kinds() {
    let constraint = RangeConstraint::new(
        ArgumentBound::Excluded(ArgumentValue::from(0_i32)),
        ArgumentBound::Included(ArgumentValue::from(10_i32)),
    );
    assert!(matches!(constraint.lower(), ArgumentBound::Excluded(_)));
    assert!(matches!(constraint.upper(), ArgumentBound::Included(_)));
}
```

Import each tested type explicitly from `qubit_argument` in its test file.

- [ ] **Step 3: Run the tests and verify the missing-type failure**

Run:

```bash
cargo test --test argument_tests argument::argument_path_tests
```

Expected: compilation fails because `ArgumentPath` and the new test modules do not yet exist at the crate root.

- [ ] **Step 4: Implement `ArgumentPath`**

Implement this public surface in `argument_path.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgumentPath(String);

impl ArgumentPath {
    pub fn new(path: &str) -> Self;
    pub fn as_str(&self) -> &str;
}

impl AsRef<str> for ArgumentPath;
impl Display for ArgumentPath;
```

`new` copies `path`; `Display` delegates to `Formatter::write_str`. Add complete English documentation to the module, type, methods, and trait implementations where behavior is not obvious.

- [ ] **Step 5: Implement `ArgumentValue`**

Implement the exact variants and primitive conversions:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgumentValue {
    Signed(i128),
    Unsigned(u128),
    Float32(u32),
    Float64(u64),
}
```

Use local macros to implement `From` for every primitive integer type. Convert `f32` and `f64` with `to_bits`. Implement custom `Debug` and `Display`; reconstruct floats with `from_bits` so negative zero, infinities, and NaN remain observable.

- [ ] **Step 6: Implement the public constraint types**

Implement:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthConstraint {
    Exact(usize),
    AtLeast(usize),
    AtMost(usize),
    InRange { min: usize, max: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonConstraint {
    EqualTo(ArgumentValue),
    NotEqualTo(ArgumentValue),
    LessThan(ArgumentValue),
    AtMost(ArgumentValue),
    GreaterThan(ArgumentValue),
    AtLeast(ArgumentValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentBound {
    Unbounded,
    Included(ArgumentValue),
    Excluded(ArgumentValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeConstraint {
    lower: ArgumentBound,
    upper: ArgumentBound,
}

impl RangeConstraint {
    pub fn new(lower: ArgumentBound, upper: ArgumentBound) -> Self;
    pub fn lower(&self) -> &ArgumentBound;
    pub fn upper(&self) -> &ArgumentBound;
    pub fn into_bounds(self) -> (ArgumentBound, ArgumentBound);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexRole {
    Element,
    Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternExpectation {
    Match,
    NoMatch,
}
```

- [ ] **Step 7: Wire modules and run focused tests**

Declare the three modules privately in `src/argument/mod.rs`, re-export their public types from `argument`, then re-export them from `src/lib.rs`. Run:

```bash
cargo test --test argument_tests argument::argument_path_tests
cargo test --test argument_tests argument::argument_value_tests
cargo test --test argument_tests argument::constraint_tests
git --no-pager diff --check
```

Expected: all three focused test groups pass and `diff --check` reports no whitespace errors.

---

### Task 2: Replace the message-only error with `ArgumentError { path, kind }`

**Files:**

- Modify: `src/argument/argument_error.rs`
- Modify: `src/argument/mod.rs`
- Modify: `src/lib.rs`
- Rewrite: `tests/argument/argument_error_tests.rs`
- Rewrite: `tests/argument/domain_error_conversion_tests.rs`

**Interfaces:**

- Consumes all vocabulary types from Task 1.
- Produces `ArgumentError`, `ArgumentErrorKind`, and `ArgumentResult<T>`.
- Temporarily keeps the old `ArgumentError::new(message)`, `message()`, and string conversions so untouched v0.3 modules and tests continue compiling; Task 7 removes them.
- Adds temporary `ArgumentError::structured(path, kind)` as the structured constructor; Task 7 renames it to the final `new(path, kind)`.

- [ ] **Step 1: Write failing structured-error tests**

Replace the old error tests with exact structural assertions:

```rust
#[test]
fn test_argument_error_exposes_structured_parts() {
    let kind = ArgumentErrorKind::Length {
        actual: 12,
        constraint: LengthConstraint::AtMost(10),
    };
    let error = ArgumentError::structured("tags", kind.clone());
    assert_eq!(error.path().as_str(), "tags");
    assert_eq!(error.kind(), &kind);
    let (path, actual_kind) = error.into_parts();
    assert_eq!(path.as_str(), "tags");
    assert_eq!(actual_kind, kind);
}

#[test]
fn test_argument_error_implements_standard_traits() {
    fn assert_traits<T: std::error::Error + Send + Sync + 'static>() {}
    assert_traits::<ArgumentError>();
    let error = ArgumentError::structured("name", ArgumentErrorKind::Blank);
    assert_eq!(error.to_string(), "argument 'name' must not be blank");
    assert!(format!("{error:?}").contains("Blank"));
}
```

- [ ] **Step 2: Write failing simulated downstream tests**

Use two local domain error types:

```rust
#[derive(Debug, PartialEq, Eq)]
enum WrappedDomainError {
    InvalidArgument(ArgumentError),
}

impl From<ArgumentError> for WrappedDomainError {
    fn from(error: ArgumentError) -> Self {
        Self::InvalidArgument(error)
    }
}

fn fail_with_missing_argument() -> Result<(), WrappedDomainError> {
    Err(ArgumentError::structured("token", ArgumentErrorKind::Missing))?;
    Ok(())
}
```

Add a second `DomainError::ZeroPoolSize` conversion that matches `path == "pool_size"` and `ArgumentErrorKind::Comparison { .. }`, falling back to `InvalidArgument(error)` for all other cases. Assert both functions use `?` without `map_err`.

- [ ] **Step 3: Run the tests and verify failure**

Run:

```bash
cargo test --test argument_tests argument::argument_error_tests
cargo test --test argument_tests argument::domain_error_conversion_tests
```

Expected: compilation fails because `ArgumentErrorKind` and `ArgumentError::structured` are missing.

- [ ] **Step 4: Implement `ArgumentErrorKind` and `ArgumentError`**

Implement these variants:

```rust
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentErrorKind {
    Missing,
    Blank,
    Empty,
    Length { actual: usize, constraint: LengthConstraint },
    Comparison { actual: ArgumentValue, constraint: ComparisonConstraint },
    Range { actual: ArgumentValue, constraint: RangeConstraint },
    InvalidLengthConstraint { constraint: LengthConstraint },
    InvalidRangeConstraint { constraint: RangeConstraint },
    NotANumber,
    Index { index: usize, size: usize, role: IndexRole },
    IndexRange { start: usize, end: usize, size: usize },
    Bounds { offset: usize, length: usize, total_length: usize },
    Pattern { pattern: String, expectation: PatternExpectation },
    Custom { code: String, message: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentError {
    path: ArgumentPath,
    kind: ArgumentErrorKind,
}
```

Add `structured`, `path`, `kind`, and `into_parts`. Implement `Display` by matching every variant and deriving text solely from structured fields. Implement `std::error::Error` with no source.

Keep the old one-argument `new`, `message()`, and `From<String>`/`From<&str>` only as documented transitional APIs that create or format `Custom { code: "legacy", message }` at path `argument`; mark them `#[doc(hidden)]`. The temporary `message()` returns `String` generated from `Display`, which keeps existing `.message().contains(...)` tests compiling without duplicating stored text. Task 7 deletes all four compatibility items.

- [ ] **Step 5: Run focused and baseline tests**

Run:

```bash
cargo test --test argument_tests argument::argument_error_tests
cargo test --test argument_tests argument::domain_error_conversion_tests
cargo test --test argument_tests
git --no-pager diff --check
```

Expected: structured-error tests pass; untouched v0.3 tests continue compiling through the temporary legacy constructor.

---

### Task 3: Rebuild numeric validation around structured comparisons and `RangeBounds`

**Files:**

- Rewrite: `src/argument/numeric_argument.rs`
- Rewrite: `tests/argument/numeric_argument_tests.rs`

**Interfaces:**

- Consumes `ArgumentError::structured`, `ArgumentValue`, `ComparisonConstraint`, `ArgumentBound`, and `RangeConstraint`.
- Produces the final `NumericArgument` method family from the design.

- [ ] **Step 1: Write failing sign and comparison tests**

Use exact kind matching rather than message substrings:

```rust
#[test]
fn test_require_positive_returns_structured_comparison_error() {
    let error = 0_i32
        .require_positive("pool_size")
        .expect_err("zero must not satisfy a positive constraint");
    assert_eq!(error.path().as_str(), "pool_size");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(0_i32),
            constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(0_i32)),
        },
    );
}

#[test]
fn test_require_at_least_preserves_value() {
    assert_eq!(
        4_u32.require_at_least("workers", 2_u32)
            .expect("valid value must be returned"),
        4_u32,
    );
}
```

Add a generic helper that instantiates all primitive integer types and covers zero, non-zero, sign, and four named comparison methods.

- [ ] **Step 2: Write failing range and float tests**

Cover standard ranges and precise float behavior:

```rust
#[test]
fn test_require_in_range_supports_standard_bounds() {
    assert_eq!(5_i32.require_in_range("value", 1..=5).expect("closed upper bound"), 5);
    assert_eq!(5_i32.require_in_range("value", 1..6).expect("excluded upper bound"), 5);
    assert_eq!(5_i32.require_in_range("value", ..=5).expect("unbounded lower"), 5);
    assert_eq!(5_i32.require_in_range("value", 5..).expect("unbounded upper"), 5);
}

#[test]
fn test_require_in_range_rejects_structurally_empty_range() {
    let error = 1_i32
        .require_in_range("value", 1..1)
        .expect_err("excluded equal upper bound must be invalid");
    assert!(matches!(error.kind(), ArgumentErrorKind::InvalidRangeConstraint { .. }));
}

#[test]
fn test_numeric_validation_rejects_nan_and_preserves_negative_zero() {
    assert!(matches!(
        f64::NAN.require_at_least("value", 0.0).expect_err("NaN must fail").kind(),
        ArgumentErrorKind::NotANumber,
    ));
    assert_eq!((-0.0_f64).require_zero("value").expect("negative zero equals zero").to_bits(), (-0.0_f64).to_bits());
}
```

- [ ] **Step 3: Run numeric tests and verify old API mismatch**

Run:

```bash
cargo test --test argument_tests argument::numeric_argument_tests
```

Expected: compilation fails on the renamed comparison methods and missing `require_in_range`.

- [ ] **Step 4: Implement the final numeric trait**

Define:

```rust
pub trait NumericArgument: Sized {
    fn require_zero(self, path: &str) -> ArgumentResult<Self>;
    fn require_non_zero(self, path: &str) -> ArgumentResult<Self>;
    fn require_positive(self, path: &str) -> ArgumentResult<Self>;
    fn require_non_negative(self, path: &str) -> ArgumentResult<Self>;
    fn require_negative(self, path: &str) -> ArgumentResult<Self>;
    fn require_non_positive(self, path: &str) -> ArgumentResult<Self>;
    fn require_less_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self>;
    fn require_greater_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self>;
    fn require_in_range<R>(self, path: &str, range: R) -> ArgumentResult<Self>
    where
        R: RangeBounds<Self>;
}
```

Use a private `NumericValue: Copy + PartialOrd` trait with `zero`, `to_argument_value`, and `is_nan`. Implement it only for supported primitives. Private helpers create comparison and range errors and have complete documentation.

For equal endpoints, accept only `Included`/`Included`; reject any pair containing `Excluded`. Reject reversed endpoints before validating the actual value. Compare without arithmetic.

Keep the old free `require_equal` and `require_not_equal` functions as hidden transitional exports until Task 7 so the crate-root export table remains compilable during the task sequence. Do not use them in new tests or documentation.

- [ ] **Step 5: Run numeric and full tests**

Run:

```bash
cargo test --test argument_tests argument::numeric_argument_tests
cargo test --test argument_tests
git --no-pager diff --check
```

Expected: numeric tests pass. Remaining old string, collection, option, and condition tests still pass through their existing APIs.

---

### Task 4: Make string validation ownership-preserving and feature-gate regex

**Files:**

- Rewrite: `src/argument/string_argument.rs`
- Rewrite: `tests/argument/string_argument_tests.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

**Interfaces:**

- Produces `StringArgument` implementations for `String` and `&str`.
- Adds `regex` feature with no default features.
- Uses `Length`, `InvalidLengthConstraint`, `Blank`, and `Pattern` errors.

- [ ] **Step 1: Update Cargo features and write failing core string tests**

Use this manifest shape:

```toml
[features]
default = []
regex = ["dep:regex"]

[dependencies]
regex = { version = "1.12", optional = true }
```

Replace string tests with ownership and Unicode behavior:

```rust
#[test]
fn test_require_non_blank_preserves_owned_string() {
    let value = String::from("qubit");
    let validated: String = value.require_non_blank("name").expect("name is non-blank");
    assert_eq!(validated, "qubit");
}

#[test]
fn test_byte_len_and_char_count_have_distinct_semantics() {
    let value = "汉😀";
    assert!(value.require_byte_len("value", 7).is_ok());
    assert!(value.require_char_count("value", 2).is_ok());
}

#[test]
fn test_string_error_does_not_expose_input() {
    let secret = "secret-token-value";
    let error = secret.require_byte_len_at_most("token", 4).expect_err("secret is too long");
    assert!(!format!("{error:?}").contains(secret));
    assert!(!error.to_string().contains(secret));
}
```

Add exact tests for all byte and char constraint methods, Unicode whitespace, invalid `min > max`, borrowed `&str`, and empty strings.

- [ ] **Step 2: Add feature-gated failing regex tests**

Guard imports and tests with `#[cfg(feature = "regex")]`:

```rust
#[cfg(feature = "regex")]
#[test]
fn test_require_match_returns_pattern_error_without_input() {
    let pattern = Regex::new("^[a-z]+$").expect("test pattern is valid");
    let error = "123".require_match("name", &pattern).expect_err("digits must fail");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Pattern {
            pattern: String::from("^[a-z]+$"),
            expectation: PatternExpectation::Match,
        },
    );
}
```

- [ ] **Step 3: Run both feature modes and verify failure**

Run:

```bash
cargo test --no-default-features --test argument_tests argument::string_argument_tests
cargo test --all-features --test argument_tests argument::string_argument_tests
```

Expected: compilation fails because the ownership-preserving and explicitly named length methods are missing.

- [ ] **Step 4: Implement the final string trait**

Define the nine core methods from the design plus feature-gated regex methods. Every method takes `self`, a `&str` path, and its numeric constraint, then returns `ArgumentResult<Self>`.

Use documented private helpers over `&str` to avoid duplicating validation logic between owned and borrowed implementations. Count bytes with `len()` and Unicode scalar values with `chars().count()`. Validate `min <= max` before checking actual length.

Under `#[cfg(feature = "regex")]`, import `regex::Regex` explicitly and implement `require_match`/`require_not_match` with `Regex::is_match`.

- [ ] **Step 5: Run string and feature tests**

Run:

```bash
cargo test --no-default-features --test argument_tests argument::string_argument_tests
cargo test --all-features --test argument_tests argument::string_argument_tests
cargo test --all-features --test argument_tests
git --no-pager diff --check
```

Expected: both feature modes pass and the default build does not require regex APIs.

---

### Task 5: Rebuild collection and Option validation without clones or null terminology

**Files:**

- Rewrite: `src/argument/collection_argument.rs`
- Rewrite: `src/argument/option_argument.rs`
- Rewrite: `tests/argument/collection_argument_tests.rs`
- Rewrite: `tests/argument/option_argument_tests.rs`

**Interfaces:**

- Produces ownership-preserving `CollectionArgument` implementations for `Vec<T>`, `&[T]`, and `[T; N]`.
- Produces `OptionArgument<T>::require_some` and `validate_if_some`.

- [ ] **Step 1: Write failing collection tests**

Use explicit type annotations to prove ownership behavior:

```rust
#[test]
fn test_require_non_empty_preserves_vec_ownership() {
    let values = vec![1, 2, 3];
    let validated: Vec<i32> = values.require_non_empty("values").expect("vector is non-empty");
    assert_eq!(validated, vec![1, 2, 3]);
}

#[test]
fn test_require_len_in_reports_invalid_constraint() {
    let error = [1, 2]
        .require_len_in("values", 3, 1)
        .expect_err("reversed length range must fail");
    assert!(matches!(error.kind(), ArgumentErrorKind::InvalidLengthConstraint { .. }));
}
```

Cover borrowed slices, arrays, empty inputs, exact, minimum, maximum, and range lengths.

- [ ] **Step 2: Write failing Option tests**

Use a non-`Clone` value and an execution flag:

```rust
#[derive(Debug, PartialEq, Eq)]
struct NonClone(u32);

#[test]
fn test_validate_if_some_preserves_non_clone_value() {
    let value = Some(NonClone(7));
    let validated = value
        .validate_if_some(|item| {
            if item.0 == 7 {
                Ok(())
            } else {
                Err(ArgumentError::structured("item", ArgumentErrorKind::Custom {
                    code: String::from("unexpected_value"),
                    message: String::from("item must equal seven"),
                }))
            }
        })
        .expect("value satisfies validator");
    assert_eq!(validated, Some(NonClone(7)));
}
```

Add a `Cell<bool>` test proving `None` does not execute the validator and a `require_some` test matching `Missing`.

- [ ] **Step 3: Run tests and verify failure**

Run:

```bash
cargo test --test argument_tests argument::collection_argument_tests
cargo test --test argument_tests argument::option_argument_tests
```

Expected: compilation fails on renamed ownership-preserving methods.

- [ ] **Step 4: Implement collection validation**

Define:

```rust
pub trait CollectionArgument: Sized {
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self>;
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self>;
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self>;
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self>;
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self>;
}
```

Use documented private helpers that accept only `(path: &str, actual: usize, constraint: LengthConstraint)` and return `ArgumentResult<()>`. Implement the trait for `Vec<T>`, `&[T]`, and `[T; N]` without cloning.

- [ ] **Step 5: Implement Option validation**

Define:

```rust
pub trait OptionArgument<T>: Sized {
    fn require_some(self, path: &str) -> ArgumentResult<T>;

    fn validate_if_some<F>(self, validator: F) -> ArgumentResult<Self>
    where
        F: FnOnce(&T) -> ArgumentResult<()>;
}
```

For `Some`, borrow the contained value only for the validator call, then return the original option. For `None`, skip the closure. Remove the old null-named trait methods from the final trait. Keep the old free `require_null_or` and `require_element_non_null` functions as hidden transitional exports until Task 7 so existing root re-exports remain compilable; do not use them in new tests.

- [ ] **Step 6: Run focused and full tests**

Run:

```bash
cargo test --test argument_tests argument::collection_argument_tests
cargo test --test argument_tests argument::option_argument_tests
cargo test --all-features --test argument_tests
git --no-pager diff --check
```

Expected: focused and full integration tests pass with substantially fewer Option tests than v0.3.

---

### Task 6: Replace condition and state helpers with bounds and custom argument validation

**Files:**

- Rename: `src/argument/condition.rs` → `src/argument/bounds.rs`
- Rename: `tests/argument/condition_tests.rs` → `tests/argument/bounds_tests.rs`
- Rewrite: `src/argument/bounds.rs`
- Rewrite: `tests/argument/bounds_tests.rs`
- Modify: `src/argument/mod.rs`
- Modify: `src/lib.rs`
- Modify: `tests/argument/mod.rs`

**Interfaces:**

- Produces `require_that`, `check_bounds`, `check_element_index`, `check_position_index`, and `check_position_range`.
- Removes all state-checking APIs.

- [ ] **Step 1: Perform the approved file moves**

Run from the crate root:

```bash
command mv src/argument/condition.rs src/argument/bounds.rs
command mv tests/argument/condition_tests.rs tests/argument/bounds_tests.rs
```

Update module declarations from `condition` to `bounds` immediately so subsequent compiler diagnostics point at the new files. Update `src/lib.rs` to re-export the new bounds functions and stop re-exporting the removed condition/state functions.

- [ ] **Step 2: Replace old tests with failing bounds tests**

Use exact return values and kinds:

```rust
#[test]
fn test_check_bounds_rejects_overflow_without_adding() {
    let error = check_bounds("buffer", usize::MAX, 1, usize::MAX)
        .expect_err("one byte cannot follow the final position");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Bounds {
            offset: usize::MAX,
            length: 1,
            total_length: usize::MAX,
        },
    );
}

#[test]
fn test_check_position_range_returns_validated_range() {
    let range = check_position_range("items", 2, 5, 8).expect("range is valid");
    assert_eq!(range, 2..5);
}

#[test]
fn test_require_that_returns_original_value() {
    let value = require_that(4_u32, "workers", |value| *value % 2 == 0, "even", "must be even")
        .expect("four is even");
    assert_eq!(value, 4);
}
```

Cover element versus position roles, start-after-end, end-after-size, empty ranges, and custom failure code/message.

- [ ] **Step 3: Run bounds tests and verify failure**

Run:

```bash
cargo test --test argument_tests argument::bounds_tests
```

Expected: compilation fails because renamed functions still expose v0.3 signatures and state helpers.

- [ ] **Step 4: Implement the final bounds functions**

Implement exact signatures:

```rust
pub fn require_that<T, F>(
    value: T,
    path: &str,
    predicate: F,
    code: &str,
    message: &str,
) -> ArgumentResult<T>
where
    F: FnOnce(&T) -> bool;

pub fn check_bounds(path: &str, offset: usize, length: usize, total_length: usize)
    -> ArgumentResult<()>;
pub fn check_element_index(path: &str, index: usize, size: usize)
    -> ArgumentResult<usize>;
pub fn check_position_index(path: &str, index: usize, size: usize)
    -> ArgumentResult<usize>;
pub fn check_position_range(path: &str, start: usize, end: usize, size: usize)
    -> ArgumentResult<Range<usize>>;
```

Check `offset > total_length` before evaluating `length > total_length - offset`. Build `Custom` strings only on predicate failure. Delete all `check_state*`, message-only condition functions, and their examples.

- [ ] **Step 5: Run bounds and full tests**

Run:

```bash
cargo test --test argument_tests argument::bounds_tests
cargo test --all-features --test argument_tests
git --no-pager diff --check
```

Expected: bounds tests and all integration tests pass.

---

### Task 7: Finalize the public API, remove transitional compatibility, and bump v0.4

**Files:**

- Modify: `src/argument/argument_error.rs`
- Modify: `src/argument/mod.rs`
- Modify: `src/lib.rs`
- Modify: all production and test call sites using `ArgumentError::structured`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

**Interfaces:**

- Makes the implementation module private.
- Renames `ArgumentError::structured(path, kind)` to final `ArgumentError::new(path, kind)`.
- Removes transitional message-only constructor and string conversions.

- [ ] **Step 1: Write a root-import smoke test**

Add to `argument_error_tests.rs`:

```rust
#[test]
fn test_crate_root_exports_v04_error_vocabulary() {
    let error = ArgumentError::new("name", ArgumentErrorKind::Blank);
    let _: &ArgumentPath = error.path();
    assert_eq!(error.to_string(), "argument 'name' must not be blank");
}
```

Keep imports exclusively from `qubit_argument::{...}`; no test may import `qubit_argument::argument`.

- [ ] **Step 2: Run the smoke test and verify constructor failure**

Run:

```bash
cargo test --test argument_tests argument::argument_error_tests::test_crate_root_exports_v04_error_vocabulary
```

Expected: compilation fails because the temporary constructor is still named `structured`.

- [ ] **Step 3: Finalize constructors and module visibility**

Rename the structured constructor to:

```rust
pub fn new(path: &str, kind: ArgumentErrorKind) -> Self
```

Update every production, test, and doctest call site. Delete the transitional message-only constructor and both `From<String>` and `From<&str>` implementations.

Delete the transitional `message()` accessor and the hidden `require_equal`, `require_not_equal`, `require_null_or`, and `require_element_non_null` functions. Their behavior is intentionally absent from v0.4.

In `src/lib.rs`, declare `mod argument;`, not `pub mod argument;`. Re-export only the approved v0.4 public types, traits, and functions. In `src/argument/mod.rs`, keep all child modules private and re-export their public items for the crate root.

- [ ] **Step 4: Remove stale APIs and verify source references**

Search Rust source and tests for the removed names:

```bash
rg -n -g '*.rs' '(check_state|check_argument_fmt|require_null_or|require_element_non_null|require_equal|require_not_equal|require_in_closed_range|require_in_open_range|require_non_null|validate_if_present|require_length_be)' src tests
```

Expected: no matches. Any match is removed or rewritten before proceeding.

- [ ] **Step 5: Bump the package version and refresh the lockfile**

Set `version = "0.4.0"` in `Cargo.toml`, then run:

```bash
cargo check --no-default-features
```

Expected: `Cargo.lock` records `qubit-argument 0.4.0` and the core crate compiles without regex.

- [ ] **Step 6: Verify both feature modes and public docs**

Run:

```bash
cargo test --no-default-features
cargo test --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features
git --no-pager diff --check
```

Expected: all commands succeed with no compatibility API remaining.

---

### Task 8: Rewrite user-facing documentation for v0.4

**Files:**

- Rewrite: `README.md`
- Rewrite: `README.zh_CN.md`
- Rewrite: `src/argument/README.md`
- Rewrite: `src/argument/README.zh_CN.md`
- Modify: crate and module rustdoc in `src/lib.rs` and `src/argument/mod.rs`
- Modify: public API rustdoc throughout `src/argument/*.rs`

**Interfaces:**

- Documents only final v0.4 root imports and method names.
- Provides a complete downstream `From<ArgumentError>` example.

- [ ] **Step 1: Replace README examples with v0.4 usage**

The English and Chinese root READMEs must both include this ownership-preserving pattern:

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

Add a downstream conversion example that implements `From<ArgumentError>` and uses `?`, and explain that callers may explicitly use `expect` for internal invariants.

- [ ] **Step 2: Document features and privacy semantics**

Document:

```toml
[dependencies]
qubit-argument = "0.4"

# Enable only when regex validation is needed.
qubit-argument = { version = "0.4", features = ["regex"] }
```

State that byte length and Unicode scalar count are distinct, regex matching is not implicitly anchored, and raw string inputs are not stored in validation errors.

- [ ] **Step 3: Rewrite module guides and rustdoc**

Remove all v0.3 names and nested public module imports. Every `Result`-returning API documents the exact failure kind. Every ownership-preserving method states that successful validation returns the original value without cloning.

- [ ] **Step 4: Verify README versions and doctests**

Run:

```bash
cargo test --doc --all-features
RS_CI_PROJECT_ROOT="$PWD" python3 .rs-ci/readme-version-check.py
rg -n 'qubit_argument::argument|qubit-argument = "0\.3"|require_non_null|check_state' README.md README.zh_CN.md src
git --no-pager diff --check
```

Expected: doctests and the version checker pass; the final search has no matches.

---

### Task 9: Run project alignment and the complete CI gate

**Files:**

- Potentially modify project CI wrapper files through `align-ci.sh`.
- Modify only files required to resolve failures introduced by this refactor.

**Interfaces:**

- Validates the complete v0.4 deliverable.

- [ ] **Step 1: Run the required alignment script**

Run:

```bash
./align-ci.sh
```

Expected: exit code 0. Review any generated changes with `git --no-pager diff -- . ':!docs/superpowers'` before continuing.

- [ ] **Step 2: Run the complete CI script**

Run:

```bash
./ci-check.sh
```

Expected: formatting, style rules, clippy, feature checks, tests, doctests, coverage, audit, package checks, and README checks all pass.

- [ ] **Step 3: Fix only refactor-related failures and rerun the full gate**

For every failure, make the smallest scoped correction, rerun the directly failing command for rapid feedback, then rerun in the required final order:

```bash
./align-ci.sh
./ci-check.sh
```

Expected: both scripts exit 0 in the same final working-tree state.

- [ ] **Step 4: Perform final scope and cleanliness review**

Run:

```bash
git status --short
git --no-pager diff --stat
git --no-pager diff --check
rg -n '(TBD|TODO|todo!\(|unimplemented!\()' src tests README.md README.zh_CN.md
```

Expected: only `rs-argument` files from this plan are modified, whitespace checks pass, and no placeholders remain. Do not commit or push.
