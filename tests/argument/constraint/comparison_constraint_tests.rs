// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for scalar comparison constraints.

use qubit_argument::ArgumentValue;
use qubit_argument::ComparisonConstraint;

/// Verifies that every comparison constraint variant retains its target value.
#[test]
fn test_comparison_constraint_preserves_target_value() {
    let target = ArgumentValue::from(8_i32);

    assert!(matches!(
        ComparisonConstraint::EqualTo(target),
        ComparisonConstraint::EqualTo(ArgumentValue::Signed(8)),
    ));
    assert!(matches!(
        ComparisonConstraint::NotEqualTo(target),
        ComparisonConstraint::NotEqualTo(ArgumentValue::Signed(8)),
    ));
    assert!(matches!(
        ComparisonConstraint::LessThan(target),
        ComparisonConstraint::LessThan(ArgumentValue::Signed(8)),
    ));
    assert!(matches!(
        ComparisonConstraint::AtMost(target),
        ComparisonConstraint::AtMost(ArgumentValue::Signed(8)),
    ));
    assert!(matches!(
        ComparisonConstraint::GreaterThan(target),
        ComparisonConstraint::GreaterThan(ArgumentValue::Signed(8)),
    ));
    assert!(matches!(
        ComparisonConstraint::AtLeast(target),
        ComparisonConstraint::AtLeast(ArgumentValue::Signed(8)),
    ));
}
