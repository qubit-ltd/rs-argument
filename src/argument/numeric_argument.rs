// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for primitive numeric arguments.

use std::ops::{
    Bound,
    RangeBounds,
};

use crate::argument::{
    ArgumentBound,
    ArgumentError,
    ArgumentErrorKind,
    ArgumentResult,
    ArgumentValue,
    ComparisonConstraint,
    RangeConstraint,
};

/// Restricts numeric validation to supported primitive numeric values.
///
/// Implementations provide the type's zero value, an exact structured error
/// representation, and NaN detection. The trait is private so arbitrary
/// partially ordered caller types cannot opt into numeric validation.
trait NumericValue: Copy + PartialOrd {
    /// Returns the zero value for this primitive numeric type.
    fn zero() -> Self;

    /// Captures this value without losing integer magnitude or floating bits.
    fn to_argument_value(self) -> ArgumentValue;

    /// Returns whether this value is a floating-point NaN.
    ///
    /// Integer implementations always return `false`.
    fn is_nan(self) -> bool;
}

/// Implements primitive numeric conversion and non-NaN behavior for integers.
macro_rules! impl_numeric_value_for_integer {
    ($($numeric_type:ty),+ $(,)?) => {
        $(
            impl NumericValue for $numeric_type {
                /// Returns integer zero.
                #[inline]
                fn zero() -> Self {
                    0
                }

                /// Captures the integer without losing its value.
                #[inline]
                fn to_argument_value(self) -> ArgumentValue {
                    ArgumentValue::from(self)
                }

                /// Reports that an integer can never be NaN.
                #[inline]
                fn is_nan(self) -> bool {
                    false
                }
            }
        )+
    };
}

impl_numeric_value_for_integer!(i8, i16, i32, i64, i128, isize);
impl_numeric_value_for_integer!(u8, u16, u32, u64, u128, usize);

impl NumericValue for f32 {
    /// Returns positive floating-point zero.
    #[inline]
    fn zero() -> Self {
        0.0
    }

    /// Captures the exact IEEE 754 bit pattern of this value.
    #[inline]
    fn to_argument_value(self) -> ArgumentValue {
        ArgumentValue::from(self)
    }

    /// Returns whether this value is NaN.
    #[inline]
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

impl NumericValue for f64 {
    /// Returns positive floating-point zero.
    #[inline]
    fn zero() -> Self {
        0.0
    }

    /// Captures the exact IEEE 754 bit pattern of this value.
    #[inline]
    fn to_argument_value(self) -> ArgumentValue {
        ArgumentValue::from(self)
    }

    /// Returns whether this value is NaN.
    #[inline]
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

/// Validates primitive numeric arguments while preserving their values.
///
/// Every successful method returns the original value without conversion or
/// normalization. Failures contain structured comparison or range data, and
/// every method rejects floating-point NaN values with
/// [`ArgumentErrorKind::NotANumber`].
pub trait NumericArgument: Sized {
    /// Requires this value to equal zero.
    ///
    /// Success returns the original value without cloning. A nonzero value
    /// returns [`ArgumentErrorKind::Comparison`] with an `EqualTo(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    fn require_zero(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value not to equal zero.
    ///
    /// Success returns the original value without cloning. Zero returns
    /// [`ArgumentErrorKind::Comparison`] with a `NotEqualTo(0)` constraint at
    /// `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    fn require_non_zero(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be strictly greater than zero.
    ///
    /// Success returns the original value without cloning. A value that is not
    /// positive returns [`ArgumentErrorKind::Comparison`] with a
    /// `GreaterThan(0)` constraint at `path`; NaN returns
    /// [`ArgumentErrorKind::NotANumber`].
    fn require_positive(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be greater than or equal to zero.
    ///
    /// Success returns the original value without cloning. A negative value
    /// returns [`ArgumentErrorKind::Comparison`] with an `AtLeast(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    fn require_non_negative(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be strictly less than zero.
    ///
    /// Success returns the original value without cloning. A value that is not
    /// negative returns [`ArgumentErrorKind::Comparison`] with a `LessThan(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    fn require_negative(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be less than or equal to zero.
    ///
    /// Success returns the original value without cloning. A positive value
    /// returns [`ArgumentErrorKind::Comparison`] with an `AtMost(0)`
    /// constraint at `path`; NaN returns [`ArgumentErrorKind::NotANumber`].
    fn require_non_positive(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this value to be strictly less than `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with a `LessThan`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    fn require_less_than(self, path: &str, bound: Self)
    -> ArgumentResult<Self>;

    /// Requires this value to be less than or equal to `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with an `AtMost`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this value to be strictly greater than `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with a
    /// `GreaterThan` constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    fn require_greater_than(
        self,
        path: &str,
        bound: Self,
    ) -> ArgumentResult<Self>;

    /// Requires this value to be greater than or equal to `bound`.
    ///
    /// Success returns the original value without cloning. An unsatisfied
    /// comparison returns [`ArgumentErrorKind::Comparison`] with an `AtLeast`
    /// constraint at `path`; a NaN value or bound returns
    /// [`ArgumentErrorKind::NotANumber`].
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self>;

    /// Requires this value to lie within `range`.
    ///
    /// Standard inclusive, exclusive, and unbounded [`RangeBounds`] are
    /// supported. The range structure is validated before this value: reversed
    /// endpoints and equal endpoints with either endpoint excluded return
    /// [`ArgumentErrorKind::InvalidRangeConstraint`]. NaN endpoints or values
    /// return [`ArgumentErrorKind::NotANumber`].
    /// An otherwise out-of-range value returns [`ArgumentErrorKind::Range`].
    /// Successful validation returns the original value without cloning.
    fn require_in_range<R>(self, path: &str, range: R) -> ArgumentResult<Self>
    where
        R: RangeBounds<Self>;
}

impl<T> NumericArgument for T
where
    T: NumericValue,
{
    /// Requires equality with zero and preserves the original value.
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

    /// Requires inequality with zero and preserves the original value.
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

    /// Requires a value strictly greater than zero.
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

    /// Requires a value greater than or equal to zero.
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

    /// Requires a value strictly less than zero.
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

    /// Requires a value less than or equal to zero.
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

    /// Requires a value strictly less than the supplied bound.
    #[inline]
    fn require_less_than(
        self,
        path: &str,
        bound: Self,
    ) -> ArgumentResult<Self> {
        validate_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::LessThan(bound.to_argument_value()),
            |actual, bound| actual < bound,
        )
    }

    /// Requires a value less than or equal to the supplied bound.
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

    /// Requires a value strictly greater than the supplied bound.
    #[inline]
    fn require_greater_than(
        self,
        path: &str,
        bound: Self,
    ) -> ArgumentResult<Self> {
        validate_comparison(
            self,
            path,
            bound,
            ComparisonConstraint::GreaterThan(bound.to_argument_value()),
            |actual, bound| actual > bound,
        )
    }

    /// Requires a value greater than or equal to the supplied bound.
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

    /// Validates the range structure before checking and returning this value.
    #[inline]
    fn require_in_range<R>(self, path: &str, range: R) -> ArgumentResult<Self>
    where
        R: RangeBounds<Self>,
    {
        let constraint = capture_range_constraint(&range);
        validate_range_structure(path, &range, &constraint)?;
        validate_not_nan(path, self)?;
        if range_contains(&range, self) {
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

/// Rejects a NaN numeric value at the supplied argument path.
///
/// `value` is inspected without normalization. Integer values always succeed;
/// floating-point NaN values return `ArgumentErrorKind::NotANumber` at `path`.
fn validate_not_nan<T>(path: &str, value: T) -> ArgumentResult<()>
where
    T: NumericValue,
{
    if value.is_nan() {
        Err(ArgumentError::new(path, ArgumentErrorKind::NotANumber))
    } else {
        Ok(())
    }
}

/// Applies one comparison and returns the unchanged numeric value on success.
///
/// `actual` and `bound` are checked for NaN before `predicate` is evaluated.
/// If the predicate returns `false`, the error records `actual` and the exact
/// supplied `constraint` at `path`.
fn validate_comparison<T, F>(
    actual: T,
    path: &str,
    bound: T,
    constraint: ComparisonConstraint,
    predicate: F,
) -> ArgumentResult<T>
where
    T: NumericValue,
    F: FnOnce(T, T) -> bool,
{
    validate_not_nan(path, actual)?;
    validate_not_nan(path, bound)?;
    if predicate(actual, bound) {
        Ok(actual)
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Comparison {
                actual: actual.to_argument_value(),
                constraint,
            },
        ))
    }
}

/// Captures a borrowed standard-library bound as a structured argument bound.
///
/// Included and excluded endpoints are converted without losing numeric bits;
/// an unbounded endpoint remains unbounded.
fn capture_argument_bound<T>(bound: Bound<&T>) -> ArgumentBound
where
    T: NumericValue,
{
    match bound {
        Bound::Unbounded => ArgumentBound::Unbounded,
        Bound::Included(value) => {
            ArgumentBound::Included(value.to_argument_value())
        }
        Bound::Excluded(value) => {
            ArgumentBound::Excluded(value.to_argument_value())
        }
    }
}

/// Captures both endpoints from `range` without validating their relationship.
///
/// The returned constraint preserves inclusive, exclusive, unbounded, and
/// floating-point bit-pattern details exactly.
fn capture_range_constraint<T, R>(range: &R) -> RangeConstraint
where
    T: NumericValue,
    R: RangeBounds<T>,
{
    RangeConstraint::new(
        capture_argument_bound(range.start_bound()),
        capture_argument_bound(range.end_bound()),
    )
}

/// Validates that `range` denotes a structurally non-empty numeric interval.
///
/// Endpoint NaNs return `NotANumber`. Reversed endpoints, or equal endpoints
/// where either bound is excluded, return `InvalidRangeConstraint` containing
/// a clone of `constraint`. Unbounded sides require no ordering comparison.
fn validate_range_structure<T, R>(
    path: &str,
    range: &R,
    constraint: &RangeConstraint,
) -> ArgumentResult<()>
where
    T: NumericValue,
    R: RangeBounds<T>,
{
    validate_range_bound_not_nan(path, range.start_bound())?;
    validate_range_bound_not_nan(path, range.end_bound())?;

    let is_valid = match (range.start_bound(), range.end_bound()) {
        (Bound::Unbounded, _) | (_, Bound::Unbounded) => return Ok(()),
        (Bound::Included(lower), Bound::Included(upper)) => lower <= upper,
        (Bound::Included(lower), Bound::Excluded(upper))
        | (Bound::Excluded(lower), Bound::Included(upper))
        | (Bound::Excluded(lower), Bound::Excluded(upper)) => lower < upper,
    };
    if is_valid {
        Ok(())
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::InvalidRangeConstraint {
                constraint: constraint.clone(),
            },
        ))
    }
}

/// Rejects a NaN endpoint while accepting unbounded and ordinary endpoints.
///
/// A NaN included or excluded endpoint returns `NotANumber` at `path`.
fn validate_range_bound_not_nan<T>(
    path: &str,
    bound: Bound<&T>,
) -> ArgumentResult<()>
where
    T: NumericValue,
{
    match bound {
        Bound::Included(value) | Bound::Excluded(value) => {
            validate_not_nan(path, *value)
        }
        Bound::Unbounded => Ok(()),
    }
}

/// Returns whether `actual` satisfies both bounds of `range`.
///
/// This helper assumes the range structure and all values were already checked
/// for NaN. It uses comparisons only and performs no endpoint arithmetic.
fn range_contains<T, R>(range: &R, actual: T) -> bool
where
    T: NumericValue,
    R: RangeBounds<T>,
{
    let satisfies_lower = match range.start_bound() {
        Bound::Unbounded => true,
        Bound::Included(lower) => actual >= *lower,
        Bound::Excluded(lower) => actual > *lower,
    };
    let satisfies_upper = match range.end_bound() {
        Bound::Unbounded => true,
        Bound::Included(upper) => actual <= *upper,
        Bound::Excluded(upper) => actual < *upper,
    };
    satisfies_lower && satisfies_upper
}
