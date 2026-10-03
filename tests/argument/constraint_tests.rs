// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for reusable argument constraints.

use qubit_argument::ArgumentBound;
use qubit_argument::ArgumentValue;
use qubit_argument::ComparisonConstraint;
use qubit_argument::IndexRole;
use qubit_argument::LengthConstraint;
use qubit_argument::PatternExpectation;
use qubit_argument::RangeConstraint;

/// Verifies that a range constraint retains inclusive and exclusive bounds.
#[test]
fn test_range_constraint_preserves_bound_kinds() {
    let constraint = RangeConstraint::new(
        ArgumentBound::Excluded(ArgumentValue::from(0_i32)),
        ArgumentBound::Included(ArgumentValue::from(10_i32)),
    );
    assert!(matches!(constraint.lower(), ArgumentBound::Excluded(_)));
    assert!(matches!(constraint.upper(), ArgumentBound::Included(_)));
}

/// Verifies that consuming a range returns both owned bounds unchanged.
#[test]
fn test_into_bounds_returns_owned_bounds() {
    let lower = ArgumentBound::Included(ArgumentValue::from(-5_i32));
    let upper = ArgumentBound::Unbounded;
    let constraint = RangeConstraint::new(lower.clone(), upper.clone());

    assert_eq!(constraint.into_bounds(), (lower, upper));
}

/// Verifies that every length constraint variant is available.
#[test]
fn test_length_constraint_exposes_all_variants() {
    assert!(matches!(LengthConstraint::Exact(3), LengthConstraint::Exact(3)));
    assert!(matches!(LengthConstraint::AtLeast(3), LengthConstraint::AtLeast(3)));
    assert!(matches!(LengthConstraint::AtMost(3), LengthConstraint::AtMost(3)));
    assert!(matches!(
        LengthConstraint::InRange { min: 2, max: 4 },
        LengthConstraint::InRange { min: 2, max: 4 }
    ));
}

/// Verifies that every comparison constraint variant retains its value.
#[test]
fn test_comparison_constraint_exposes_all_variants() {
    let value = ArgumentValue::from(7_i32);
    assert!(matches!(
        ComparisonConstraint::EqualTo(value),
        ComparisonConstraint::EqualTo(ArgumentValue::Signed(7)),
    ));
    assert!(matches!(
        ComparisonConstraint::NotEqualTo(value),
        ComparisonConstraint::NotEqualTo(ArgumentValue::Signed(7)),
    ));
    assert!(matches!(
        ComparisonConstraint::LessThan(value),
        ComparisonConstraint::LessThan(ArgumentValue::Signed(7)),
    ));
    assert!(matches!(
        ComparisonConstraint::AtMost(value),
        ComparisonConstraint::AtMost(ArgumentValue::Signed(7)),
    ));
    assert!(matches!(
        ComparisonConstraint::GreaterThan(value),
        ComparisonConstraint::GreaterThan(ArgumentValue::Signed(7)),
    ));
    assert!(matches!(
        ComparisonConstraint::AtLeast(value),
        ComparisonConstraint::AtLeast(ArgumentValue::Signed(7)),
    ));
}

/// Verifies that index roles distinguish element and position indexes.
#[test]
fn test_index_role_distinguishes_index_kinds() {
    assert_ne!(IndexRole::Element, IndexRole::Position);
}

/// Verifies that pattern expectations distinguish matches from non-matches.
#[test]
fn test_pattern_expectation_distinguishes_outcomes() {
    assert_ne!(PatternExpectation::Match, PatternExpectation::NoMatch);
}
