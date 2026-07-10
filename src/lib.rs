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
pub mod argument;

pub use argument::{
    ArgumentBound,
    ArgumentError,
    ArgumentErrorKind,
    ArgumentPath,
    ArgumentResult,
    ArgumentValue,
    CollectionArgument,
    ComparisonConstraint,
    IndexRole,
    LengthConstraint,
    NumericArgument,
    OptionArgument,
    PatternExpectation,
    RangeConstraint,
    StringArgument,
    check_argument,
    check_argument_fmt,
    check_argument_with_message,
    check_bounds,
    check_element_index,
    check_position_index,
    check_position_indexes,
    check_state,
    check_state_with_message,
    require_element_non_null,
    require_equal,
    require_not_equal,
    require_null_or,
};
