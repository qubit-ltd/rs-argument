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
/// # Examples
///
/// ```
/// use qubit_argument::{ArgumentBound, ArgumentValue, RangeConstraint};
///
/// let range = RangeConstraint::new(
///     ArgumentBound::Included(ArgumentValue::from(1_i32)),
///     ArgumentBound::Excluded(ArgumentValue::from(5_i32)),
/// );
/// assert!(matches!(range.lower(), ArgumentBound::Included(_)));
/// ```
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
    /// The lower endpoint, including whether it is inclusive, exclusive, or
    /// absent.
    lower: ArgumentBound,
    /// The upper endpoint, including whether it is inclusive, exclusive, or
    /// absent.
    upper: ArgumentBound,
}

impl RangeConstraint {
    /// Creates a range from its lower and upper bounds.
    ///
    /// The bounds are retained exactly as supplied and are not ordered or
    /// otherwise validated.
    ///
    /// # Parameters
    ///
    /// * `lower` - The lower endpoint and its inclusion mode.
    /// * `upper` - The upper endpoint and its inclusion mode.
    ///
    /// # Returns
    ///
    /// The range containing the supplied bounds.
    #[inline]
    pub fn new(lower: ArgumentBound, upper: ArgumentBound) -> Self {
        Self { lower, upper }
    }

    /// Returns the lower bound of this range.
    ///
    /// # Returns
    ///
    /// A reference to the lower endpoint and its inclusion mode.
    #[must_use = "the caller should inspect the lower range bound"]
    #[inline]
    pub fn lower(&self) -> &ArgumentBound {
        &self.lower
    }

    /// Returns the upper bound of this range.
    ///
    /// # Returns
    ///
    /// A reference to the upper endpoint and its inclusion mode.
    #[must_use = "the caller should inspect the upper range bound"]
    #[inline]
    pub fn upper(&self) -> &ArgumentBound {
        &self.upper
    }

    /// Consumes this range and returns its lower and upper bounds.
    ///
    /// The first tuple element is the lower bound and the second is the upper
    /// bound.
    ///
    /// # Returns
    ///
    /// The owned lower and upper bounds, in that order.
    #[must_use = "the caller should inspect the range bounds"]
    #[inline]
    pub fn into_bounds(self) -> (ArgumentBound, ArgumentBound) {
        let Self { lower, upper } = self;
        (lower, upper)
    }
}
