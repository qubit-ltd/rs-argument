// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Reusable vocabulary describing argument validation constraints.

use crate::argument::ArgumentValue;

/// A constraint on the length of a string or collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthConstraint {
    /// Requires exactly the specified length.
    Exact(usize),
    /// Requires at least the specified length.
    AtLeast(usize),
    /// Requires at most the specified length.
    AtMost(usize),
    /// Requires a length between `min` and `max`, inclusive.
    InRange {
        /// The inclusive minimum length.
        min: usize,
        /// The inclusive maximum length.
        max: usize,
    },
}

/// A comparison between an argument and a captured numeric value.
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

/// One side of a numeric range constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentBound {
    /// Places no limit on this side of the range.
    Unbounded,
    /// Includes the captured endpoint in the range.
    Included(ArgumentValue),
    /// Excludes the captured endpoint from the range.
    Excluded(ArgumentValue),
}

/// A numeric range with independently inclusive, exclusive, or absent bounds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeConstraint {
    lower: ArgumentBound,
    upper: ArgumentBound,
}

impl RangeConstraint {
    /// Creates a range from its lower and upper bounds.
    ///
    /// The bounds are retained exactly as supplied and are not ordered or
    /// otherwise validated.
    #[inline]
    pub fn new(lower: ArgumentBound, upper: ArgumentBound) -> Self {
        Self { lower, upper }
    }

    /// Returns the lower bound of this range.
    #[inline]
    pub fn lower(&self) -> &ArgumentBound {
        &self.lower
    }

    /// Returns the upper bound of this range.
    #[inline]
    pub fn upper(&self) -> &ArgumentBound {
        &self.upper
    }

    /// Consumes this range and returns its lower and upper bounds.
    ///
    /// The first tuple element is the lower bound and the second is the upper
    /// bound.
    #[inline]
    pub fn into_bounds(self) -> (ArgumentBound, ArgumentBound) {
        let Self { lower, upper } = self;
        (lower, upper)
    }
}

/// The role of an index in an indexed argument operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexRole {
    /// Identifies an existing element and therefore excludes the collection end.
    Element,
    /// Identifies an insertion or boundary position, including the collection end.
    Position,
}

/// Whether a string is expected to match a pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternExpectation {
    /// Requires the string to match the pattern.
    Match,
    /// Requires the string not to match the pattern.
    NoMatch,
}
