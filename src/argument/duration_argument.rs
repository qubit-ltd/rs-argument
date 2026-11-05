// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for duration arguments.

use std::time::Duration;

use crate::argument::ArgumentResult;
use crate::argument::ArgumentValue;
use crate::argument::ComparisonConstraint;
use crate::argument::internal::Sealed;
use crate::argument::internal::duration_validation::validate_duration_comparison;

/// Validates standard-library duration arguments without losing their unit.
///
/// Every successful method returns the original duration. Comparison failures
/// retain both the actual duration and bound as unit-bearing
/// [`ArgumentValue`] values.
///
/// The trait is sealed and implemented only for [`Duration`].
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use qubit_argument::DurationArgument;
///
/// let timeout = Duration::from_secs(5).require_positive("timeout").unwrap();
/// assert_eq!(timeout, Duration::from_secs(5));
/// ```
pub trait DurationArgument: Sealed + Sized {
    /// Requires this duration to be strictly greater than zero.
    ///
    /// Success returns the original duration. A zero duration returns
    /// [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) with a `GreaterThan(Duration::ZERO)`
    /// constraint at `path`.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is nonzero.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is zero.
    fn require_positive(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this duration to be strictly less than `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) at `path` with the exact
    /// duration bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The exclusive upper bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is less than `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is greater
    /// than or equal to `bound`.
    fn require_less_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this duration to be less than or equal to `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) at `path` with the exact
    /// duration bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The inclusive upper bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is less than or equal to `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration exceeds
    /// `bound`.
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this duration to be strictly greater than `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) at `path` with the exact
    /// duration bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The exclusive lower bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is greater than `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is less
    /// than or equal to `bound`.
    fn require_greater_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this duration to be greater than or equal to `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) at `path` with the exact
    /// duration bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The inclusive lower bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is greater than or equal to `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is less
    /// than `bound`.
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self>;
}

impl DurationArgument for Duration {
    /// Requires a nonzero duration and preserves its exact value.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is nonzero.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is zero.
    #[inline]
    fn require_positive(self, path: &str) -> ArgumentResult<Self> {
        validate_duration_comparison(
            self,
            path,
            Duration::ZERO,
            ComparisonConstraint::GreaterThan(ArgumentValue::from(Duration::ZERO)),
            |actual, bound| actual > bound,
        )
    }

    /// Requires a duration strictly below the supplied bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The exclusive upper bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is less than `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is greater
    /// than or equal to `bound`.
    #[inline]
    fn require_less_than(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_duration_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::LessThan(ArgumentValue::from(bound)),
            |actual, bound| actual < bound,
        )
    }

    /// Requires a duration at or below the supplied bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The inclusive upper bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is less than or equal to `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration exceeds
    /// `bound`.
    #[inline]
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_duration_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::AtMost(ArgumentValue::from(bound)),
            |actual, bound| actual <= bound,
        )
    }

    /// Requires a duration strictly above the supplied bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The exclusive lower bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is greater than `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is less than
    /// or equal to `bound`.
    #[inline]
    fn require_greater_than(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_duration_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::GreaterThan(ArgumentValue::from(bound)),
            |actual, bound| actual > bound,
        )
    }

    /// Requires a duration at or above the supplied bound.
    ///
    /// # Parameters
    ///
    /// - `path`: Identifies the argument in a validation error.
    /// - `bound`: The inclusive lower bound for the duration.
    ///
    /// # Returns
    ///
    /// The unchanged duration when it is greater than or equal to `bound`.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Comparison`](crate::argument::ArgumentErrorKind::Comparison) when the duration is less than
    /// `bound`.
    #[inline]
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_duration_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::AtLeast(ArgumentValue::from(bound)),
            |actual, bound| actual >= bound,
        )
    }
}
