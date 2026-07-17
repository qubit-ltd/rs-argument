// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric range constraints.

use crate::argument::ArgumentBound;

/// A numeric range with independently inclusive, exclusive, or absent bounds.
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_argument::{ArgumentBound, RangeConstraint};
///
/// RangeConstraint::new(ArgumentBound::Unbounded, ArgumentBound::Unbounded);
/// ```
#[must_use]
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
    #[inline(always)]
    pub fn lower(&self) -> &ArgumentBound {
        &self.lower
    }

    /// Returns the upper bound of this range.
    #[inline(always)]
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
