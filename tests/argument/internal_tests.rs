// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for behavior backed by private argument internals.

use qubit_argument::ArgumentErrorKind;
use qubit_argument::ArgumentValue;
use qubit_argument::ComparisonConstraint;
use qubit_argument::NumericArgument;

/// Verifies that private numeric internals preserve the public integer
/// validation contract.
#[test]
fn test_internal_numeric_support_preserves_integer_contract() {
    let error = 0_i32
        .require_positive("count")
        .expect_err("zero is not positive");

    assert_eq!(error.path().as_str(), "count");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(0_i32),
            constraint: ComparisonConstraint::GreaterThan(
                ArgumentValue::from(0_i32),
            ),
        },
    );
}
