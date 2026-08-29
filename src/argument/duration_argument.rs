// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for duration arguments.

use std::time::Duration;

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::ArgumentValue;
use crate::argument::ComparisonConstraint;
use crate::argument::internal::Sealed;

/// Validates standard-library duration arguments without losing their unit.
///
/// Every successful method returns the original duration. Comparison failures
/// retain both the actual duration and bound as unit-bearing
/// [`ArgumentValue`] values.
///
/// The trait is sealed and implemented only for [`Duration`].
pub trait DurationArgument: Sealed + Sized {
    /// Requires this duration to be strictly greater than zero.
    ///
    /// Success returns the original duration. A zero duration returns
    /// [`ArgumentErrorKind::Comparison`] with a `GreaterThan(Duration::ZERO)`
    /// constraint at `path`.
    fn require_positive(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this duration to be strictly less than `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`] at `path` with the exact
    /// duration bound.
    fn require_less_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this duration to be less than or equal to `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`] at `path` with the exact
    /// duration bound.
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this duration to be strictly greater than `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`] at `path` with the exact
    /// duration bound.
    fn require_greater_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this duration to be greater than or equal to `bound`.
    ///
    /// Success returns the original duration. An unsatisfied comparison
    /// returns [`ArgumentErrorKind::Comparison`] at `path` with the exact
    /// duration bound.
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self>;
}

impl DurationArgument for Duration {
    /// Requires a nonzero duration and preserves its exact value.
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

/// Applies one duration comparison and preserves the actual value on success.
///
/// `actual` and `bound` are passed to `predicate` exactly once. If the
/// predicate returns `false`, the error records `actual`, `constraint`, and
/// `path` without converting the duration to a unitless number.
#[inline]
fn validate_duration_comparison<F>(
    actual: Duration,
    path: &str,
    bound: Duration,
    constraint: ComparisonConstraint,
    predicate: F,
) -> ArgumentResult<Duration>
where
    F: FnOnce(Duration, Duration) -> bool,
{
    if predicate(actual, bound) {
        Ok(actual)
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(actual),
                constraint,
            },
        ))
    }
}
