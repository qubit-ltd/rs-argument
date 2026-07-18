// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for length constraint values.

use qubit_argument::LengthConstraint;

/// Verifies that exact, lower, upper, and range constraints are distinct.
#[test]
fn test_length_constraint_distinguishes_relationships() {
    assert_ne!(LengthConstraint::Exact(3), LengthConstraint::AtLeast(3));
    assert_ne!(LengthConstraint::AtLeast(3), LengthConstraint::AtMost(3));
    assert_ne!(
        LengthConstraint::AtMost(3),
        LengthConstraint::InRange { min: 2, max: 3 },
    );
}

/// Verifies that range constraints preserve their inclusive endpoints.
#[test]
fn test_length_constraint_in_range_preserves_endpoints() {
    let constraint = LengthConstraint::InRange { min: 2, max: 5 };

    assert!(matches!(
        constraint,
        LengthConstraint::InRange { min: 2, max: 5 },
    ));
}
