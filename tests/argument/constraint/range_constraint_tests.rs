// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for numeric range constraints.

use qubit_argument::ArgumentBound;
use qubit_argument::ArgumentValue;
use qubit_argument::RangeConstraint;

/// Verifies that a range constraint exposes its lower and upper bounds.
#[test]
fn test_range_constraint_exposes_bounds_by_reference() {
    let lower = ArgumentBound::Included(ArgumentValue::from(1_i32));
    let upper = ArgumentBound::Excluded(ArgumentValue::from(4_i32));
    let constraint = RangeConstraint::new(lower.clone(), upper.clone());

    assert_eq!(constraint.lower(), &lower);
    assert_eq!(constraint.upper(), &upper);
}

/// Verifies that a range constraint returns its owned bounds in lower-upper
/// order.
#[test]
fn test_range_constraint_into_bounds_preserves_order() {
    let constraint = RangeConstraint::new(
        ArgumentBound::Unbounded,
        ArgumentBound::Included(ArgumentValue::from(10_i32)),
    );

    assert_eq!(
        constraint.into_bounds(),
        (
            ArgumentBound::Unbounded,
            ArgumentBound::Included(ArgumentValue::from(10_i32)),
        ),
    );
}
