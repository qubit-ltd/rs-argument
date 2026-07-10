// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured kinds of argument validation failure.

use crate::argument::{
    ArgumentValue,
    ComparisonConstraint,
    IndexRole,
    LengthConstraint,
    LengthMetric,
    PatternExpectation,
    RangeConstraint,
};

/// Identifies the validation rule that an argument failed.
///
/// Each variant stores only structured context needed to inspect and format
/// the failure. Validated string contents are never captured implicitly.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentErrorKind {
    /// The required argument was absent.
    Missing,
    /// The string argument contained no non-whitespace characters.
    Blank,
    /// The collection argument was empty.
    Empty,
    /// The argument length did not satisfy a length constraint.
    Length {
        /// The observed length in the unit identified by `metric`.
        actual: usize,
        /// The required length relationship.
        constraint: LengthConstraint,
        /// The unit used to measure the observed and required lengths.
        metric: LengthMetric,
    },
    /// The numeric argument did not satisfy a comparison constraint.
    Comparison {
        /// The observed numeric value.
        actual: ArgumentValue,
        /// The required comparison relationship.
        constraint: ComparisonConstraint,
    },
    /// The numeric argument was outside a required range.
    Range {
        /// The observed numeric value.
        actual: ArgumentValue,
        /// The required range.
        constraint: RangeConstraint,
    },
    /// The supplied length constraint was internally invalid.
    InvalidLengthConstraint {
        /// The invalid length constraint.
        constraint: LengthConstraint,
        /// The unit to which the invalid constraint would have applied.
        metric: LengthMetric,
    },
    /// The supplied numeric range was internally invalid.
    InvalidRangeConstraint {
        /// The invalid numeric range.
        constraint: RangeConstraint,
    },
    /// The numeric argument was a floating-point NaN value.
    NotANumber,
    /// An element or position index was outside its valid domain.
    Index {
        /// The rejected index.
        index: usize,
        /// The collection size used for validation.
        size: usize,
        /// Whether the index identifies an element or a position.
        role: IndexRole,
    },
    /// A position range was invalid for a collection size.
    IndexRange {
        /// The inclusive start position.
        start: usize,
        /// The exclusive end position.
        end: usize,
        /// The collection size used for validation.
        size: usize,
    },
    /// An offset and length did not fit within a total length.
    Bounds {
        /// The rejected starting offset.
        offset: usize,
        /// The rejected span length.
        length: usize,
        /// The available total length.
        total_length: usize,
    },
    /// A string did not satisfy a pattern expectation.
    Pattern {
        /// The pattern text used for validation.
        pattern: String,
        /// Whether a match or non-match was required.
        expectation: PatternExpectation,
    },
    /// A caller-defined validation rule failed.
    Custom {
        /// A machine-readable caller-defined code.
        code: String,
        /// A human-readable caller-defined explanation.
        message: String,
    },
}
