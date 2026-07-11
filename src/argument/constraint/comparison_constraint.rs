// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric comparison constraints.

use crate::argument::ArgumentValue;

/// A comparison between an argument and a captured scalar value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonConstraint {
    /// Requires equality with the captured value.
    EqualTo(ArgumentValue),
    /// Requires inequality with the captured value.
    NotEqualTo(ArgumentValue),
    /// Requires a value strictly less than the captured value.
    LessThan(ArgumentValue),
    /// Requires a value less than or equal to the captured value.
    AtMost(ArgumentValue),
    /// Requires a value strictly greater than the captured value.
    GreaterThan(ArgumentValue),
    /// Requires a value greater than or equal to the captured value.
    AtLeast(ArgumentValue),
}
