// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Qubit Argument
//!
//! Ownership-preserving validation for arguments, configuration values,
//! durations, indexes, and bounds.
//!
//! All public traits, functions, and error types are exported from the crate
//! root. Validation returns [`ArgumentResult`] instead of panicking, and each
//! failure contains an owned [`ArgumentPath`] plus an inspectable
//! [`ArgumentErrorKind`]. Successful validation returns the original owned
//! value or borrow without cloning it.
//!
//! Validation extension traits are sealed and implemented only for the types
//! documented by this crate. [`ArgumentErrorKind`], [`ArgumentValue`], and
//! [`LengthMetric`] are non-exhaustive; downstream matches must include a
//! wildcard arm so the structured vocabulary can evolve compatibly.
//!
//! # Ownership-preserving validation
//!
//! ```rust
//! use qubit_argument::{ArgumentResult, NumericArgument, StringArgument};
//!
//! fn validate_user(age: u8, name: String) -> ArgumentResult<(u8, String)> {
//!     let age = age.require_in_range("age", 0..=150)?;
//!     let name = name
//!         .require_non_blank("name")?
//!         .require_char_count_in("name", 3, 32)?;
//!     Ok((age, name))
//! }
//!
//! let (age, name) = validate_user(42, String::from("Ada"))?;
//! assert_eq!((age, name.as_str()), (42, "Ada"));
//! # Ok::<(), qubit_argument::ArgumentError>(())
//! ```
//!
//! # Downstream errors
//!
//! A downstream error can implement `From<ArgumentError>` and then use `?`
//! without a `map_err` adapter:
//!
//! ```rust
//! use qubit_argument::{ArgumentError, NumericArgument};
//!
//! #[derive(Debug)]
//! enum DomainError {
//!     InvalidArgument(ArgumentError),
//! }
//!
//! impl From<ArgumentError> for DomainError {
//!     fn from(error: ArgumentError) -> Self {
//!         Self::InvalidArgument(error)
//!     }
//! }
//!
//! fn validate_pool_size(size: u32) -> Result<u32, DomainError> {
//!     let size = size.require_positive("pool_size")?;
//!     Ok(size)
//! }
//!
//! assert_eq!(validate_pool_size(4)?, 4);
//! # Ok::<(), DomainError>(())
//! ```
//!
//! Callers that are checking a genuine internal invariant may instead choose
//! to call `expect` with a meaningful explanation. The validation APIs do not
//! make that recovery-versus-panic decision on the caller's behalf.
//!
//! # Nested configuration and durations
//!
//! Nested validators can report local paths and add parent context only on
//! failure. Durations retain their unit in structured comparison errors:
//!
//! ```rust
//! use std::time::Duration;
//!
//! use qubit_argument::{
//!     ArgumentResult,
//!     ArgumentResultExt,
//!     DurationArgument,
//! };
//!
//! fn validate_timeouts(connect: Duration) -> ArgumentResult<()> {
//!     connect.require_positive("connect")?;
//!     Ok(())
//! }
//!
//! let error = validate_timeouts(Duration::ZERO)
//!     .with_path_prefix("timeouts")
//!     .expect_err("a zero connection timeout is invalid");
//! assert_eq!(error.path().as_str(), "timeouts.connect");
//! ```
//!
//! # Strings and optional regex support
//!
//! Byte-length methods measure UTF-8 bytes. Character-count methods measure
//! Unicode scalar values, not grapheme clusters. String validation errors do
//! not retain the inspected input string. Structured length errors retain a
//! [`LengthMetric`] so byte length, Unicode scalar count, and collection
//! element count remain distinguishable.
//!
//! The default feature set is empty. Enabling the `regex` feature adds
//! `StringArgument::require_match` and `StringArgument::require_not_match`.
//! They use `Regex::is_match` semantics and do not implicitly anchor patterns.

/// Argument validation implementations re-exported at the crate root.
mod argument;

pub use argument::ArgumentBound;
pub use argument::ArgumentError;
pub use argument::ArgumentErrorKind;
pub use argument::ArgumentPath;
pub use argument::ArgumentResult;
pub use argument::ArgumentResultExt;
pub use argument::ArgumentValue;
pub use argument::CollectionArgument;
pub use argument::ComparisonConstraint;
pub use argument::DurationArgument;
pub use argument::FloatArgument;
pub use argument::IndexRole;
pub use argument::LengthConstraint;
pub use argument::LengthMetric;
pub use argument::NumericArgument;
pub use argument::OptionArgument;
pub use argument::PatternExpectation;
pub use argument::RangeConstraint;
pub use argument::StringArgument;
pub use argument::check_bounds;
pub use argument::check_element_index;
pub use argument::check_position_index;
pub use argument::check_position_range;
pub use argument::require_that;
