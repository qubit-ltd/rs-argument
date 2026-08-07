// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for public behavior provided by private numeric value support.

use qubit_argument::ArgumentErrorKind;
use qubit_argument::ArgumentValue;
use qubit_argument::ComparisonConstraint;
use qubit_argument::NumericArgument;

/// Verifies that integer numeric support captures the signed value exactly.
#[test]
fn test_numeric_value_support_captures_signed_integer() {
    let error = (-1_i32)
        .require_non_negative("offset")
        .expect_err("negative offset must fail");

    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(-1_i32),
            constraint: ComparisonConstraint::AtLeast(
                ArgumentValue::from(0_i32),
            ),
        },
    );
}

/// Verifies that floating-point numeric support classifies NaN values.
#[test]
fn test_numeric_value_support_classifies_nan() {
    let error = f64::NAN
        .require_zero("ratio")
        .expect_err("NaN cannot satisfy numeric validation");

    assert_eq!(error.kind(), &ArgumentErrorKind::NotANumber);
}
