// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for ownership-preserving numeric argument validation.

use std::fmt::Debug;
use std::ops::Bound;

use qubit_argument::{
    ArgumentBound, ArgumentError, ArgumentErrorKind, ArgumentValue, ComparisonConstraint,
    NumericArgument, RangeConstraint,
};

/// Asserts that an error has the requested path and structured kind.
fn assert_structured_error(
    error: ArgumentError,
    expected_path: &str,
    expected_kind: ArgumentErrorKind,
) {
    assert_eq!(error.path().as_str(), expected_path);
    assert_eq!(error.kind(), &expected_kind);
}

/// Exercises the common integer validators for one primitive integer type.
///
/// `zero`, `one`, and `two` must be the numeric values zero, one, and two for
/// `T`. Every successful validation is required to return its original value.
fn assert_integer_validators<T>(zero: T, one: T, two: T)
where
    T: NumericArgument + Copy + Debug + PartialEq,
{
    assert_eq!(zero.require_zero("value").expect("zero is valid"), zero);
    assert_eq!(one.require_non_zero("value").expect("one is non-zero"), one,);
    assert_eq!(one.require_positive("value").expect("one is positive"), one,);
    assert_eq!(
        zero.require_non_negative("value")
            .expect("zero is non-negative"),
        zero,
    );
    assert!(zero.require_negative("value").is_err());
    assert_eq!(
        zero.require_non_positive("value")
            .expect("zero is non-positive"),
        zero,
    );
    assert_eq!(
        one.require_less_than("value", two)
            .expect("one is less than two"),
        one,
    );
    assert_eq!(
        one.require_at_most("value", one)
            .expect("one is at most one"),
        one,
    );
    assert_eq!(
        two.require_greater_than("value", one)
            .expect("two is greater than one"),
        two,
    );
    assert_eq!(
        one.require_at_least("value", one)
            .expect("one is at least one"),
        one,
    );
}

/// Asserts that every numeric validator rejects a NaN value or bound.
fn assert_nan_is_rejected<T>(nan: T, zero: T, one: T)
where
    T: NumericArgument + Copy + Debug,
{
    let actual_nan_results = [
        nan.require_zero("value"),
        nan.require_non_zero("value"),
        nan.require_positive("value"),
        nan.require_non_negative("value"),
        nan.require_negative("value"),
        nan.require_non_positive("value"),
        nan.require_less_than("value", one),
        nan.require_at_most("value", one),
        nan.require_greater_than("value", zero),
        nan.require_at_least("value", zero),
        nan.require_in_range("value", zero..=one),
    ];
    for result in actual_nan_results {
        assert_structured_error(
            result.expect_err("NaN actual value must fail"),
            "value",
            ArgumentErrorKind::NotANumber,
        );
    }

    let nan_bound_results = [
        zero.require_less_than("value", nan),
        zero.require_at_most("value", nan),
        one.require_greater_than("value", nan),
        one.require_at_least("value", nan),
        zero.require_in_range("value", (Bound::Included(nan), Bound::Included(one))),
        zero.require_in_range("value", (Bound::Included(zero), Bound::Included(nan))),
    ];
    for result in nan_bound_results {
        assert_structured_error(
            result.expect_err("NaN bound must fail"),
            "value",
            ArgumentErrorKind::NotANumber,
        );
    }
}

/// Verifies that every primitive integer type implements the complete trait.
#[test]
fn test_require_methods_support_all_primitive_integer_types() {
    assert_integer_validators(0_i8, 1_i8, 2_i8);
    assert_integer_validators(0_i16, 1_i16, 2_i16);
    assert_integer_validators(0_i32, 1_i32, 2_i32);
    assert_integer_validators(0_i64, 1_i64, 2_i64);
    assert_integer_validators(0_i128, 1_i128, 2_i128);
    assert_integer_validators(0_isize, 1_isize, 2_isize);
    assert_integer_validators(0_u8, 1_u8, 2_u8);
    assert_integer_validators(0_u16, 1_u16, 2_u16);
    assert_integer_validators(0_u32, 1_u32, 2_u32);
    assert_integer_validators(0_u64, 1_u64, 2_u64);
    assert_integer_validators(0_u128, 1_u128, 2_u128);
    assert_integer_validators(0_usize, 1_usize, 2_usize);
}

/// Verifies the structured equality error emitted for a non-zero value.
#[test]
fn test_require_zero_returns_structured_comparison_error() {
    let error = 1_i32
        .require_zero("value")
        .expect_err("one must not satisfy a zero constraint");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(1_i32),
            constraint: ComparisonConstraint::EqualTo(ArgumentValue::from(0_i32)),
        },
    );
}

/// Verifies the structured inequality error emitted for zero.
#[test]
fn test_require_non_zero_returns_structured_comparison_error() {
    let error = 0_u32
        .require_non_zero("workers")
        .expect_err("zero must not satisfy a non-zero constraint");
    assert_structured_error(
        error,
        "workers",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(0_u32),
            constraint: ComparisonConstraint::NotEqualTo(ArgumentValue::from(0_u32)),
        },
    );
}

/// Verifies the strict positive constraint and its argument path.
#[test]
fn test_require_positive_returns_structured_comparison_error() {
    let error = 0_i32
        .require_positive("pool_size")
        .expect_err("zero must not satisfy a positive constraint");
    assert_structured_error(
        error,
        "pool_size",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(0_i32),
            constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(0_i32)),
        },
    );
}

/// Verifies the inclusive lower bound used by non-negative validation.
#[test]
fn test_require_non_negative_returns_structured_comparison_error() {
    let error = (-1_i32)
        .require_non_negative("offset")
        .expect_err("negative values must fail");
    assert_structured_error(
        error,
        "offset",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(-1_i32),
            constraint: ComparisonConstraint::AtLeast(ArgumentValue::from(0_i32)),
        },
    );
}

/// Verifies the strict negative comparison used by negative validation.
#[test]
fn test_require_negative_returns_structured_comparison_error() {
    let error = 0_i32
        .require_negative("delta")
        .expect_err("zero must not be negative");
    assert_structured_error(
        error,
        "delta",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(0_i32),
            constraint: ComparisonConstraint::LessThan(ArgumentValue::from(0_i32)),
        },
    );
}

/// Verifies the inclusive upper bound used by non-positive validation.
#[test]
fn test_require_non_positive_returns_structured_comparison_error() {
    let error = 1_u32
        .require_non_positive("delta")
        .expect_err("positive values must fail");
    assert_structured_error(
        error,
        "delta",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(1_u32),
            constraint: ComparisonConstraint::AtMost(ArgumentValue::from(0_u32)),
        },
    );
}

/// Verifies the strict upper comparison error.
#[test]
fn test_require_less_than_returns_structured_comparison_error() {
    let error = 5_i32
        .require_less_than("value", 5)
        .expect_err("equal values must fail a strict upper comparison");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(5_i32),
            constraint: ComparisonConstraint::LessThan(ArgumentValue::from(5_i32)),
        },
    );
}

/// Verifies the inclusive upper comparison error.
#[test]
fn test_require_at_most_returns_structured_comparison_error() {
    let error = 6_i32
        .require_at_most("value", 5)
        .expect_err("six exceeds the inclusive upper bound");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(6_i32),
            constraint: ComparisonConstraint::AtMost(ArgumentValue::from(5_i32)),
        },
    );
}

/// Verifies the strict lower comparison error.
#[test]
fn test_require_greater_than_returns_structured_comparison_error() {
    let error = 5_i32
        .require_greater_than("value", 5)
        .expect_err("equal values must fail a strict lower comparison");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(5_i32),
            constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(5_i32)),
        },
    );
}

/// Verifies that inclusive lower comparison returns the original value.
#[test]
fn test_require_at_least_preserves_value() {
    assert_eq!(
        4_u32
            .require_at_least("workers", 2_u32)
            .expect("valid value must be returned"),
        4_u32,
    );
}

/// Verifies the inclusive lower comparison error.
#[test]
fn test_require_at_least_returns_structured_comparison_error() {
    let error = 4_i32
        .require_at_least("value", 5)
        .expect_err("four is below the inclusive lower bound");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(4_i32),
            constraint: ComparisonConstraint::AtLeast(ArgumentValue::from(5_i32)),
        },
    );
}

/// Verifies standard inclusive, exclusive, and unbounded range syntax.
#[test]
fn test_require_in_range_supports_standard_bounds() {
    assert_eq!(
        5_i32
            .require_in_range("value", 1..=5)
            .expect("closed upper bound"),
        5,
    );
    assert_eq!(
        5_i32
            .require_in_range("value", 1..6)
            .expect("excluded upper bound"),
        5,
    );
    assert_eq!(
        5_i32
            .require_in_range("value", ..=5)
            .expect("unbounded lower"),
        5,
    );
    assert_eq!(
        5_i32
            .require_in_range("value", 5..)
            .expect("unbounded upper"),
        5,
    );
    assert_eq!(
        5_i32
            .require_in_range("value", ..)
            .expect("fully unbounded range"),
        5,
    );
}

/// Verifies that an out-of-range value produces an exact range constraint.
#[test]
fn test_require_in_range_returns_structured_range_error() {
    let error = 6_i32
        .require_in_range("value", 1..=5)
        .expect_err("six is outside the range");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Range {
            actual: ArgumentValue::from(6_i32),
            constraint: RangeConstraint::new(
                ArgumentBound::Included(ArgumentValue::from(1_i32)),
                ArgumentBound::Included(ArgumentValue::from(5_i32)),
            ),
        },
    );
}

/// Verifies that a singleton range is valid when both endpoints are included.
#[test]
fn test_require_in_range_accepts_closed_singleton() {
    assert_eq!(
        1_i32
            .require_in_range("value", 1..=1)
            .expect("a closed singleton range is valid"),
        1,
    );
}

/// Verifies that an excluded upper endpoint makes equal bounds empty.
#[test]
fn test_require_in_range_rejects_included_excluded_singleton() {
    let error = 1_i32
        .require_in_range("value", 1..1)
        .expect_err("an excluded equal upper bound is empty");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::InvalidRangeConstraint {
            constraint: RangeConstraint::new(
                ArgumentBound::Included(ArgumentValue::from(1_i32)),
                ArgumentBound::Excluded(ArgumentValue::from(1_i32)),
            ),
        },
    );
}

/// Verifies that an excluded lower endpoint makes equal bounds empty.
#[test]
fn test_require_in_range_rejects_excluded_included_singleton() {
    let error = 1_i32
        .require_in_range("value", (Bound::Excluded(1_i32), Bound::Included(1_i32)))
        .expect_err("an excluded equal lower bound is empty");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::InvalidRangeConstraint {
            constraint: RangeConstraint::new(
                ArgumentBound::Excluded(ArgumentValue::from(1_i32)),
                ArgumentBound::Included(ArgumentValue::from(1_i32)),
            ),
        },
    );
}

/// Verifies that two excluded equal endpoints make an empty range.
#[test]
fn test_require_in_range_rejects_excluded_singleton() {
    let error = 1_i32
        .require_in_range("value", (Bound::Excluded(1_i32), Bound::Excluded(1_i32)))
        .expect_err("two excluded equal endpoints are empty");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::InvalidRangeConstraint {
            constraint: RangeConstraint::new(
                ArgumentBound::Excluded(ArgumentValue::from(1_i32)),
                ArgumentBound::Excluded(ArgumentValue::from(1_i32)),
            ),
        },
    );
}

/// Verifies that reversed endpoints are rejected before checking the value.
#[test]
fn test_require_in_range_rejects_reversed_bounds_before_actual() {
    let error = f64::NAN
        .require_in_range("value", 2.0..=1.0)
        .expect_err("the reversed range must be rejected first");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::InvalidRangeConstraint {
            constraint: RangeConstraint::new(
                ArgumentBound::Included(ArgumentValue::from(2.0_f64)),
                ArgumentBound::Included(ArgumentValue::from(1.0_f64)),
            ),
        },
    );
}

/// Verifies full-width integer ranges without endpoint arithmetic.
#[test]
fn test_require_in_range_supports_primitive_extremes() {
    assert_eq!(
        i128::MIN
            .require_in_range("value", i128::MIN..=i128::MAX)
            .expect("signed minimum lies in the full range"),
        i128::MIN,
    );
    assert_eq!(
        u128::MAX
            .require_in_range("value", 0_u128..=u128::MAX)
            .expect("unsigned maximum lies in the full range"),
        u128::MAX,
    );
}

/// Verifies structured NaN rejection for both float widths and all methods.
#[test]
fn test_require_methods_reject_nan() {
    assert_nan_is_rejected(f32::NAN, 0.0_f32, 1.0_f32);
    assert_nan_is_rejected(f64::NAN, 0.0_f64, 1.0_f64);
}

/// Verifies that successful validation preserves the negative-zero bit pattern.
#[test]
fn test_require_zero_preserves_negative_zero() {
    let validated_f32 = (-0.0_f32)
        .require_zero("value")
        .expect("negative zero equals zero");
    assert_eq!(validated_f32.to_bits(), (-0.0_f32).to_bits());

    let validated_f64 = (-0.0_f64)
        .require_zero("value")
        .expect("negative zero equals zero");
    assert_eq!(validated_f64.to_bits(), (-0.0_f64).to_bits());
}

/// Verifies that negative zero remains distinguishable in structured errors.
#[test]
fn test_require_positive_preserves_negative_zero_in_error() {
    let error = (-0.0_f64)
        .require_positive("value")
        .expect_err("negative zero is not positive");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(-0.0_f64),
            constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(0.0_f64)),
        },
    );
}

/// Verifies that infinities participate in comparisons and unbounded ranges.
#[test]
fn test_require_methods_support_float_infinities() {
    assert_eq!(
        f64::INFINITY
            .require_greater_than("value", f64::MAX)
            .expect("positive infinity exceeds finite values"),
        f64::INFINITY,
    );
    assert_eq!(
        f64::NEG_INFINITY
            .require_less_than("value", f64::MIN)
            .expect("negative infinity precedes finite values"),
        f64::NEG_INFINITY,
    );
    assert_eq!(
        f32::INFINITY
            .require_in_range("value", ..)
            .expect("an unbounded range accepts infinity"),
        f32::INFINITY,
    );
}
