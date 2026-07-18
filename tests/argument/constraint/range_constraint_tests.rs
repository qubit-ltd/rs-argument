// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for numeric range constraints.

use qubit_argument::{
    ArgumentBound,
    ArgumentValue,
    RangeConstraint,
};

/// Verifies that a range constraint exposes its lower and upper bounds.
#[test]
fn test_range_constraint_exposes_bounds_by_reference() {
    let constraint = RangeConstraint::new(
        ArgumentBound::Included(ArgumentValue::from(1_i32)),
        ArgumentBound::Excluded(ArgumentValue::from(4_i32)),
    );

    assert!(matches!(constraint.lower(), ArgumentBound::Included(_)));
    assert!(matches!(constraint.upper(), ArgumentBound::Excluded(_)));
}

/// Verifies that a range constraint returns its owned bounds in lower-upper
/// order.
#[test]
fn test_range_constraint_into_bounds_preserves_order() {
    let lower = ArgumentBound::Unbounded;
    let upper = ArgumentBound::Included(ArgumentValue::from(10_i32));
    let constraint = RangeConstraint::new(lower.clone(), upper.clone());

    assert_eq!(constraint.into_bounds(), (lower, upper));
}
