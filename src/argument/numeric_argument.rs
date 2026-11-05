// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for primitive numeric arguments.

use std::ops::RangeBounds;

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::ComparisonConstraint;
use crate::argument::internal::NumericValue;
use crate::argument::internal::Sealed;
use crate::argument::internal::capture_range_constraint;
use crate::argument::internal::range_contains;
use crate::argument::internal::snapshot_range_bounds;
use crate::argument::internal::validate_comparison;
use crate::argument::internal::validate_not_nan;
use crate::argument::internal::validate_range_structure;

/// Validates primitive numeric arguments while preserving their values.
///
/// Every successful method returns the original value without conversion or
/// normalization. Failures contain structured comparison or range data, and
/// every method rejects floating-point NaN values with
/// [`ArgumentErrorKind::NotANumber`].
///
/// The trait is sealed and implemented only for primitive numeric types.
///
/// # Examples
///
/// ```
/// use qubit_argument::NumericArgument;
///
/// let value = 5_i32.require_in_range("value", 1..=10).unwrap();
/// assert_eq!(value, 5);
/// ```
pub trait NumericArgument: Sealed + Sized {
    /// Requires this value to equal zero.
    ///
    /// Success returns the original value without cloning. A nonzero value
    /// returns [`ArgumentErrorKind::Comparison`] with an `EqualTo(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it equals zero.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is nonzero, or `NotANumber` for NaN.
    fn require_zero(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value not to equal zero.
    ///
    /// Success returns the original value without cloning. Zero returns
    /// [`ArgumentErrorKind::Comparison`] with a `NotEqualTo(0)` constraint at
    /// `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is nonzero.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is zero, or `NotANumber` for NaN.
    fn require_non_zero(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be strictly greater than zero.
    ///
    /// Success returns the original value without cloning. A value that is not
    /// positive returns [`ArgumentErrorKind::Comparison`] with a
    /// `GreaterThan(0)` constraint at `path`; NaN returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is positive.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is not positive, or `NotANumber` for
    /// NaN.
    fn require_positive(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be greater than or equal to zero.
    ///
    /// Success returns the original value without cloning. A negative value
    /// returns [`ArgumentErrorKind::Comparison`] with an `AtLeast(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is non-negative.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is negative, or `NotANumber` for
    /// NaN.
    fn require_non_negative(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be strictly less than zero.
    ///
    /// Success returns the original value without cloning. A value that is not
    /// negative returns [`ArgumentErrorKind::Comparison`] with a `LessThan(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is negative.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is not negative, or `NotANumber` for
    /// NaN.
    fn require_negative(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be less than or equal to zero.
    ///
    /// Success returns the original value without cloning. A positive value
    /// returns [`ArgumentErrorKind::Comparison`] with an `AtMost(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is non-positive.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is positive, or `NotANumber` for
    /// NaN.
    fn require_non_positive(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be strictly less than `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with a `LessThan`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is less than `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    fn require_less_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this value to be less than or equal to `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with an `AtMost`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is at most `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this value to be strictly greater than `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with a
    /// `GreaterThan` constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is greater than `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    fn require_greater_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this value to be greater than or equal to `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with an `AtLeast`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is at least `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this value to lie within `range`.
    ///
    /// Standard inclusive, exclusive, and unbounded [`RangeBounds`] are
    /// supported. The range structure is validated before this value: reversed
    /// endpoints and equal endpoints with either endpoint excluded return
    /// [`ArgumentErrorKind::InvalidRangeConstraint`]. NaN endpoints or values
    /// return [`ArgumentErrorKind::NotANumber`].
    /// An otherwise out-of-range value returns [`ArgumentErrorKind::Range`].
    /// The range's start and end accessors are each called exactly once, and
    /// all checks use that owned endpoint snapshot.
    /// Successful validation returns the original value without cloning.
    ///
    /// # Type Parameters
    ///
    /// - `R`: A range exposing bounds of this numeric type.
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `range`: Inclusive, exclusive, or unbounded validation interval.
    ///
    /// # Returns
    ///
    /// The unchanged value when it satisfies the validated range.
    ///
    /// # Errors
    ///
    /// Returns `InvalidRangeConstraint` for an invalid range, `Range` when the
    /// value is outside it, or `NotANumber` for a NaN value or endpoint.
    fn require_in_range<R>(self, path: &str, range: R) -> ArgumentResult<Self>
    where
        R: RangeBounds<Self>;
}

impl<T> NumericArgument for T
where
    T: NumericValue,
{
    /// Requires this value to equal zero.
    ///
    /// Success returns the original value without cloning. A nonzero value
    /// returns [`ArgumentErrorKind::Comparison`] with an `EqualTo(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it equals zero.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is nonzero, or `NotANumber` for NaN.
    #[inline]
    fn require_zero(self, path: &str) -> ArgumentResult<Self> {
        let zero = T::zero();
        validate_comparison(
            self,
            path,
            zero,
            ComparisonConstraint::EqualTo(zero.to_argument_value()),
            |actual, bound| actual == bound,
        )
    }

    /// Requires this value not to equal zero.
    ///
    /// Success returns the original value without cloning. Zero returns
    /// [`ArgumentErrorKind::Comparison`] with a `NotEqualTo(0)` constraint at
    /// `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is nonzero.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is zero, or `NotANumber` for NaN.
    #[inline]
    fn require_non_zero(self, path: &str) -> ArgumentResult<Self> {
        let zero = T::zero();
        validate_comparison(
            self,
            path,
            zero,
            ComparisonConstraint::NotEqualTo(zero.to_argument_value()),
            |actual, bound| actual != bound,
        )
    }

    /// Requires this value to be strictly greater than zero.
    ///
    /// Success returns the original value without cloning. A value that is not
    /// positive returns [`ArgumentErrorKind::Comparison`] with a
    /// `GreaterThan(0)` constraint at `path`; NaN returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is positive.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is not positive, or `NotANumber` for
    /// NaN.
    #[inline]
    fn require_positive(self, path: &str) -> ArgumentResult<Self> {
        let zero = T::zero();
        validate_comparison(
            self,
            path,
            zero,
            ComparisonConstraint::GreaterThan(zero.to_argument_value()),
            |actual, bound| actual > bound,
        )
    }

    /// Requires this value to be greater than or equal to zero.
    ///
    /// Success returns the original value without cloning. A negative value
    /// returns [`ArgumentErrorKind::Comparison`] with an `AtLeast(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is non-negative.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is negative, or `NotANumber` for
    /// NaN.
    #[inline]
    fn require_non_negative(self, path: &str) -> ArgumentResult<Self> {
        let zero = T::zero();
        validate_comparison(
            self,
            path,
            zero,
            ComparisonConstraint::AtLeast(zero.to_argument_value()),
            |actual, bound| actual >= bound,
        )
    }

    /// Requires this value to be strictly less than zero.
    ///
    /// Success returns the original value without cloning. A value that is not
    /// negative returns [`ArgumentErrorKind::Comparison`] with a `LessThan(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is negative.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is not negative, or `NotANumber` for
    /// NaN.
    #[inline]
    fn require_negative(self, path: &str) -> ArgumentResult<Self> {
        let zero = T::zero();
        validate_comparison(
            self,
            path,
            zero,
            ComparisonConstraint::LessThan(zero.to_argument_value()),
            |actual, bound| actual < bound,
        )
    }

    /// Requires this value to be less than or equal to zero.
    ///
    /// Success returns the original value without cloning. A positive value
    /// returns [`ArgumentErrorKind::Comparison`] with an `AtMost(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is non-positive.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the value is positive, or `NotANumber` for
    /// NaN.
    #[inline]
    fn require_non_positive(self, path: &str) -> ArgumentResult<Self> {
        let zero = T::zero();
        validate_comparison(
            self,
            path,
            zero,
            ComparisonConstraint::AtMost(zero.to_argument_value()),
            |actual, bound| actual <= bound,
        )
    }

    /// Requires this value to be strictly less than `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with a `LessThan`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is less than `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    #[inline]
    fn require_less_than(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::LessThan(bound.to_argument_value()),
            |actual, bound| actual < bound,
        )
    }

    /// Requires this value to be less than or equal to `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with an `AtMost`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is at most `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    #[inline]
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::AtMost(bound.to_argument_value()),
            |actual, bound| actual <= bound,
        )
    }

    /// Requires this value to be strictly greater than `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with a
    /// `GreaterThan` constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is greater than `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    #[inline]
    fn require_greater_than(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::GreaterThan(bound.to_argument_value()),
            |actual, bound| actual > bound,
        )
    }

    /// Requires this value to be greater than or equal to `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with an `AtLeast`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `bound`: Comparison boundary.
    ///
    /// # Returns
    ///
    /// The unchanged value when it is at least `bound`.
    ///
    /// # Errors
    ///
    /// Returns `Comparison` when the comparison fails, or `NotANumber` when the
    /// value or bound is NaN.
    #[inline]
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self> {
        validate_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::AtLeast(bound.to_argument_value()),
            |actual, bound| actual >= bound,
        )
    }

    /// Requires this value to lie within `range`.
    ///
    /// Standard inclusive, exclusive, and unbounded [`RangeBounds`] are
    /// supported. The range structure is validated before this value: reversed
    /// endpoints and equal endpoints with either endpoint excluded return
    /// [`ArgumentErrorKind::InvalidRangeConstraint`]. NaN endpoints or values
    /// return [`ArgumentErrorKind::NotANumber`].
    /// An otherwise out-of-range value returns [`ArgumentErrorKind::Range`].
    /// The range's start and end accessors are each called exactly once, and
    /// all checks use that owned endpoint snapshot.
    /// Successful validation returns the original value without cloning.
    ///
    /// # Type Parameters
    ///
    /// - `R`: A range exposing bounds of this numeric type.
    ///
    /// # Parameters
    ///
    /// - `path`: Location used to identify a validation failure.
    /// - `range`: Inclusive, exclusive, or unbounded validation interval.
    ///
    /// # Returns
    ///
    /// The unchanged value when it satisfies the validated range.
    ///
    /// # Errors
    ///
    /// Returns `InvalidRangeConstraint` for an invalid range, `Range` when the
    /// value is outside it, or `NotANumber` for a NaN value or endpoint.
    #[inline]
    fn require_in_range<R>(self, path: &str, range: R) -> ArgumentResult<Self>
    where
        R: RangeBounds<Self>,
    {
        let (lower_bound, upper_bound) = snapshot_range_bounds(&range);
        let constraint = capture_range_constraint(lower_bound, upper_bound);
        validate_range_structure(path, lower_bound, upper_bound, &constraint)?;
        validate_not_nan(path, self)?;
        if range_contains(lower_bound, upper_bound, self) {
            Ok(self)
        } else {
            Err(ArgumentError::new(
                path,
                ArgumentErrorKind::Range {
                    actual: self.to_argument_value(),
                    constraint,
                },
            ))
        }
    }
}
