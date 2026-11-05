// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Relationships used by string and collection length constraints.

/// A numeric relationship required of a measured string or collection length.
///
/// [`LengthMetric`](super::LengthMetric) identifies the measurement unit in a
/// structured validation error.
///
/// # Examples
///
/// ```
/// use qubit_argument::LengthConstraint;
///
/// let constraint = LengthConstraint::InRange { min: 2, max: 5 };
/// assert!(matches!(constraint, LengthConstraint::InRange { min: 2, max: 5 }));
/// ```
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_argument::LengthConstraint;
///
/// LengthConstraint::Exact(1);
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthConstraint {
    /// Requires exactly the specified length.
    Exact(
        /// The required length, measured in the unit selected by the validator.
        usize,
    ),
    /// Requires at least the specified length.
    AtLeast(
        /// The inclusive lower bound, measured in the unit selected by the
        /// validator.
        usize,
    ),
    /// Requires at most the specified length.
    AtMost(
        /// The inclusive upper bound, measured in the unit selected by the
        /// validator.
        usize,
    ),
    /// Requires a length between `min` and `max`, inclusive.
    ///
    /// Construction does not check the bounds. String and collection validators
    /// reject `min > max` as
    /// [`ArgumentErrorKind::InvalidLengthConstraint`](super::super::ArgumentErrorKind::InvalidLengthConstraint)
    /// before checking the measured length.
    InRange {
        /// The inclusive minimum length.
        min: usize,
        /// The inclusive maximum length.
        max: usize,
    },
}
