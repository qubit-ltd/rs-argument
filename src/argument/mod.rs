// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Argument validation implementation.
//!
//! This module is private. Its public items are re-exported directly from
//! `qubit_argument`, which is the only supported import path.
//!
//! # Validation API
//!
//! - [`NumericArgument`] validates primitive integers and floats, including
//!   comparison and standard `RangeBounds` checks.
//! - [`StringArgument`] distinguishes UTF-8 byte length from Unicode scalar
//!   count. With the `regex` feature, it also provides unanchored
//!   `Regex::is_match` validation.
//! - [`CollectionArgument`] validates `Vec<T>`, `&[T]`, and arrays.
//! - [`OptionArgument`] extracts required values or validates present values by
//!   shared borrow.
//! - [`require_that`] applies a caller-defined predicate.
//! - [`check_bounds`], [`check_element_index`], [`check_position_index`], and
//!   [`check_position_range`] validate slice-style bounds and indexes.
//!
//! Ownership-preserving methods return the original value or borrow on
//! success without cloning it. Every failure is an [`ArgumentError`] with an
//! [`ArgumentPath`] and a structured [`ArgumentErrorKind`]. String validation
//! errors never retain the inspected input string.
//!
//! # Error and constraint vocabulary
//!
//! [`ArgumentError::path`], [`ArgumentError::kind`], and
//! [`ArgumentError::into_parts`] expose a failure without parsing its display
//! text. [`ArgumentValue`] losslessly captures primitive numeric values.
//! [`LengthConstraint`], [`ComparisonConstraint`], [`ArgumentBound`], and
//! [`RangeConstraint`] describe failed constraints. [`IndexRole`] distinguishes
//! element indexes from boundary positions, while [`PatternExpectation`]
//! distinguishes required regex matches from required non-matches.

mod argument_error;
mod argument_error_kind;
mod argument_path;
mod argument_value;
mod bounds;
mod collection_argument;
mod constraint;
mod numeric_argument;
mod option_argument;
mod string_argument;

pub use argument_error::{
    ArgumentError,
    ArgumentResult,
};
pub use argument_error_kind::ArgumentErrorKind;
pub use argument_path::ArgumentPath;
pub use argument_value::ArgumentValue;
pub use bounds::{
    check_bounds,
    check_element_index,
    check_position_index,
    check_position_range,
    require_that,
};
pub use collection_argument::CollectionArgument;
pub use constraint::{
    ArgumentBound,
    ComparisonConstraint,
    IndexRole,
    LengthConstraint,
    PatternExpectation,
    RangeConstraint,
};
pub use numeric_argument::NumericArgument;
pub use option_argument::OptionArgument;
pub use string_argument::StringArgument;
