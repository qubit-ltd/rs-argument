// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Internal comparison and range validation for primitive numeric arguments.

use std::ops::Bound;
use std::ops::RangeBounds;

use crate::argument::ArgumentBound;
use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::ComparisonConstraint;
use crate::argument::RangeConstraint;
use crate::argument::internal::NumericValue;

/// Rejects a NaN numeric value or constraint at the supplied argument path.
///
/// `value` is inspected without normalization. Integer values always succeed;
/// floating-point NaN values return `ArgumentErrorKind::NotANumber` at `path`.
///
/// # Type Parameters
///
/// - `T`: A supported primitive numeric type.
///
/// # Parameters
///
/// - `path`: Location used for a NaN failure.
/// - `value`: Numeric value to inspect.
///
/// # Returns
///
/// Unit when the value is not NaN.
///
/// # Errors
///
/// Returns `NotANumber` when `value` is NaN.
#[inline]
pub(in crate::argument) fn validate_not_nan<T>(path: &str, value: T) -> ArgumentResult<()>
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
///
/// # Type Parameters
///
/// - `T`: A supported primitive numeric type.
/// - `F`: A predicate called once after both operands pass NaN validation.
///
/// # Parameters
///
/// - `actual`: Value to validate and return unchanged.
/// - `path`: Location used for failures.
/// - `bound`: Comparison operand.
/// - `constraint`: Structured description retained on comparison failure.
/// - `predicate`: Comparison applied to `actual` and `bound`.
///
/// # Returns
///
/// The unchanged `actual` when the comparison succeeds.
///
/// # Errors
///
/// Returns `NotANumber` for a NaN operand, or `Comparison` when the predicate
/// fails.
pub(in crate::argument) fn validate_comparison<T, F>(
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

/// Copies a borrowed standard-library bound into an owned bound.
///
/// Included and excluded endpoints are copied exactly; an unbounded endpoint
/// remains unbounded.
///
/// # Type Parameters
///
/// - `T`: A supported, copyable primitive numeric type.
///
/// # Parameters
///
/// - `bound`: Borrowed endpoint to copy.
///
/// # Returns
///
/// An owned bound retaining the endpoint and inclusion mode.
#[must_use]
#[inline]
fn copy_range_bound<T>(bound: Bound<&T>) -> Bound<T>
where
    T: NumericValue,
{
    match bound {
        Bound::Unbounded => Bound::Unbounded,
        Bound::Included(value) => Bound::Included(*value),
        Bound::Excluded(value) => Bound::Excluded(*value),
    }
}

/// Reads each endpoint from `range` once and returns an owned snapshot.
///
/// The lower and upper endpoint accessors are each invoked exactly once. The
/// returned values are reused for constraint construction, validation, and
/// membership checks.
///
/// # Type Parameters
///
/// - `T`: A supported, copyable primitive numeric type.
/// - `R`: A range exposing bounds of `T`.
///
/// # Parameters
///
/// - `range`: Range whose endpoint accessors are each called once.
///
/// # Returns
///
/// Owned lower and upper bounds in that order.
#[must_use]
#[inline]
pub(in crate::argument) fn snapshot_range_bounds<T, R>(range: &R) -> (Bound<T>, Bound<T>)
where
    T: NumericValue,
    R: RangeBounds<T>,
{
    let lower_bound = copy_range_bound(range.start_bound());
    let upper_bound = copy_range_bound(range.end_bound());
    (lower_bound, upper_bound)
}

/// Captures an owned standard-library bound as a structured argument bound.
///
/// Included and excluded endpoints are converted without losing numeric bits;
/// an unbounded endpoint remains unbounded.
///
/// # Type Parameters
///
/// - `T`: A supported primitive numeric type.
///
/// # Parameters
///
/// - `bound`: Owned endpoint to represent in an argument constraint.
///
/// # Returns
///
/// The matching structured endpoint, preserving its inclusion mode.
#[inline]
fn capture_argument_bound<T>(bound: Bound<T>) -> ArgumentBound
where
    T: NumericValue,
{
    match bound {
        Bound::Unbounded => ArgumentBound::Unbounded,
        Bound::Included(value) => ArgumentBound::Included(value.to_argument_value()),
        Bound::Excluded(value) => ArgumentBound::Excluded(value.to_argument_value()),
    }
}

/// Captures both owned endpoints without validating their relationship.
///
/// The returned constraint preserves inclusive, exclusive, unbounded, and
/// floating-point bit-pattern details exactly.
///
/// # Type Parameters
///
/// - `T`: A supported primitive numeric type.
///
/// # Parameters
///
/// - `lower_bound`: Owned lower endpoint.
/// - `upper_bound`: Owned upper endpoint.
///
/// # Returns
///
/// A structured constraint preserving both endpoints.
#[inline]
pub(in crate::argument) fn capture_range_constraint<T>(lower_bound: Bound<T>, upper_bound: Bound<T>) -> RangeConstraint
where
    T: NumericValue,
{
    RangeConstraint::new(capture_argument_bound(lower_bound), capture_argument_bound(upper_bound))
}

/// Validates that two snapshotted bounds form a non-empty numeric interval.
///
/// Endpoint NaNs return `NotANumber`. Reversed endpoints, or equal endpoints
/// where either bound is excluded, return `InvalidRangeConstraint` containing
/// a clone of `constraint`. Unbounded sides require no ordering comparison.
///
/// # Type Parameters
///
/// - `T`: A supported primitive numeric type.
///
/// # Parameters
///
/// - `path`: Location used for failures.
/// - `lower_bound`: Snapshotted lower endpoint.
/// - `upper_bound`: Snapshotted upper endpoint.
/// - `constraint`: Description cloned only for an invalid-range error.
///
/// # Returns
///
/// Unit when the endpoints describe a valid interval.
///
/// # Errors
///
/// Returns `NotANumber` for a NaN endpoint or `InvalidRangeConstraint` for an
/// empty or reversed interval.
pub(in crate::argument) fn validate_range_structure<T>(
    path: &str,
    lower_bound: Bound<T>,
    upper_bound: Bound<T>,
    constraint: &RangeConstraint,
) -> ArgumentResult<()>
where
    T: NumericValue,
{
    validate_range_bound_not_nan(path, lower_bound)?;
    validate_range_bound_not_nan(path, upper_bound)?;

    let is_valid = match (lower_bound, upper_bound) {
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
///
/// # Type Parameters
///
/// - `T`: A supported primitive numeric type.
///
/// # Parameters
///
/// - `path`: Location used for a NaN failure.
/// - `bound`: Endpoint to inspect; unbounded endpoints always succeed.
///
/// # Returns
///
/// Unit for an unbounded or non-NaN endpoint.
///
/// # Errors
///
/// Returns `NotANumber` for a NaN endpoint.
#[inline]
fn validate_range_bound_not_nan<T>(path: &str, bound: Bound<T>) -> ArgumentResult<()>
where
    T: NumericValue,
{
    match bound {
        Bound::Included(value) | Bound::Excluded(value) => validate_not_nan(path, value),
        Bound::Unbounded => Ok(()),
    }
}

/// Returns whether `actual` satisfies two snapshotted bounds.
///
/// This helper assumes the bounds and all values were already checked for NaN.
/// It uses comparisons only and performs no endpoint arithmetic.
///
/// # Type Parameters
///
/// - `T`: A supported primitive numeric type.
///
/// # Parameters
///
/// - `lower_bound`: Validated lower endpoint.
/// - `upper_bound`: Validated upper endpoint.
/// - `actual`: Non-NaN value to check.
///
/// # Returns
///
/// `true` when both bounds admit `actual`; otherwise, `false`.
#[must_use]
#[inline]
pub(in crate::argument) fn range_contains<T>(lower_bound: Bound<T>, upper_bound: Bound<T>, actual: T) -> bool
where
    T: NumericValue,
{
    let satisfies_lower = match lower_bound {
        Bound::Unbounded => true,
        Bound::Included(lower) => actual >= lower,
        Bound::Excluded(lower) => actual > lower,
    };
    let satisfies_upper = match upper_bound {
        Bound::Unbounded => true,
        Bound::Included(upper) => actual <= upper,
        Bound::Excluded(upper) => actual < upper,
    };
    satisfies_lower && satisfies_upper
}
