// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for finite floating-point argument validation.

use qubit_argument::{
    ArgumentErrorKind,
    ArgumentValue,
    FloatArgument,
};

/// Verifies that finite 32-bit and 64-bit values are returned unchanged.
#[test]
fn test_require_finite_preserves_finite_values() {
    assert_eq!(
        (-0.0_f32)
            .require_finite("factor")
            .expect("negative zero is finite")
            .to_bits(),
        (-0.0_f32).to_bits(),
    );
    assert_eq!(
        1.5_f64
            .require_finite("factor")
            .expect("one and a half is finite"),
        1.5,
    );
}

/// Verifies that positive infinity retains its exact 32-bit representation.
#[test]
fn test_require_finite_rejects_positive_infinity() {
    let error = f32::INFINITY
        .require_finite("jitter.factor")
        .expect_err("positive infinity is not finite");
    assert_eq!(error.path().as_str(), "jitter.factor");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::NotFinite {
            actual: ArgumentValue::from(f32::INFINITY),
        },
    );
}

/// Verifies that negative infinity retains its exact 64-bit representation.
#[test]
fn test_require_finite_rejects_negative_infinity() {
    let error = f64::NEG_INFINITY
        .require_finite("multiplier")
        .expect_err("negative infinity is not finite");
    assert_eq!(error.path().as_str(), "multiplier");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::NotFinite {
            actual: ArgumentValue::from(f64::NEG_INFINITY),
        },
    );
}

/// Verifies that NaN keeps the established not-a-number classification.
#[test]
fn test_require_finite_reports_not_a_number_for_nan() {
    let error = f64::NAN
        .require_finite("multiplier")
        .expect_err("NaN is not a number");
    assert_eq!(error.path().as_str(), "multiplier");
    assert_eq!(error.kind(), &ArgumentErrorKind::NotANumber);
}
