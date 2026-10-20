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
    let exact = LengthConstraint::Exact(3);
    let minimum = LengthConstraint::AtLeast(3);
    let maximum = LengthConstraint::AtMost(3);
    let range = LengthConstraint::InRange { min: 2, max: 4 };

    assert_ne!(exact, minimum);
    assert_ne!(exact, maximum);
    assert_ne!(exact, range);
    assert_ne!(minimum, maximum);
    assert_ne!(minimum, range);
    assert_ne!(maximum, range);
}

/// Verifies that every comparison constraint variant retains its value.
#[test]
fn test_comparison_constraint_exposes_all_variants() {
    let value = ArgumentValue::from(7_i32);
    let equal_to = ComparisonConstraint::EqualTo(value);
    let not_equal_to = ComparisonConstraint::NotEqualTo(value);
    let less_than = ComparisonConstraint::LessThan(value);
    let at_most = ComparisonConstraint::AtMost(value);
    let greater_than = ComparisonConstraint::GreaterThan(value);
    let at_least = ComparisonConstraint::AtLeast(value);

    assert_ne!(equal_to, not_equal_to);
    assert_ne!(equal_to, less_than);
    assert_ne!(equal_to, at_most);
    assert_ne!(equal_to, greater_than);
    assert_ne!(equal_to, at_least);
    assert_ne!(not_equal_to, less_than);
    assert_ne!(not_equal_to, at_most);
    assert_ne!(not_equal_to, greater_than);
    assert_ne!(not_equal_to, at_least);
    assert_ne!(less_than, at_most);
    assert_ne!(less_than, greater_than);
    assert_ne!(less_than, at_least);
    assert_ne!(at_most, greater_than);
    assert_ne!(at_most, at_least);
    assert_ne!(greater_than, at_least);
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
