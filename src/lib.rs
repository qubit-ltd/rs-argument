// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Qubit Argument
//!
//! Provides argument and state validation helpers for Rust applications.

/// Argument validation traits, functions, and error types.
mod argument;

pub use argument::{
    ArgumentBound, ArgumentError, ArgumentErrorKind, ArgumentPath, ArgumentResult, ArgumentValue,
    CollectionArgument, ComparisonConstraint, IndexRole, LengthConstraint, NumericArgument,
    OptionArgument, PatternExpectation, RangeConstraint, StringArgument, check_bounds,
    check_element_index, check_position_index, check_position_range, require_that,
};
