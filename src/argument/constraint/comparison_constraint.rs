// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Scalar comparison constraints.

use crate::argument::ArgumentValue;

/// A comparison between an argument and a captured scalar value.
///
/// # Examples
///
/// ```
/// use qubit_argument::ArgumentValue;
/// use qubit_argument::ComparisonConstraint;
///
/// let constraint = ComparisonConstraint::AtLeast(ArgumentValue::from(10_i32));
/// assert!(matches!(constraint, ComparisonConstraint::AtLeast(_)));
/// ```
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_argument::ArgumentValue;
/// use qubit_argument::ComparisonConstraint;
///
/// ComparisonConstraint::EqualTo(ArgumentValue::Signed(1));
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonConstraint {
    /// Requires equality with the captured value.
    EqualTo(
        /// The owned scalar value the argument must equal.
        ArgumentValue,
    ),
    /// Requires inequality with the captured value.
    NotEqualTo(
        /// The owned scalar value the argument must differ from.
        ArgumentValue,
    ),
    /// Requires a value strictly less than the captured value.
    LessThan(
        /// The owned exclusive upper bound for the argument.
        ArgumentValue,
    ),
    /// Requires a value less than or equal to the captured value.
    AtMost(
        /// The owned inclusive upper bound for the argument.
        ArgumentValue,
    ),
    /// Requires a value strictly greater than the captured value.
    GreaterThan(
        /// The owned exclusive lower bound for the argument.
        ArgumentValue,
    ),
    /// Requires a value greater than or equal to the captured value.
    AtLeast(
        /// The owned inclusive lower bound for the argument.
        ArgumentValue,
    ),
}
