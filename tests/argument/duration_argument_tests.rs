// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured duration argument validation.

use std::time::Duration;

use qubit_argument::ArgumentError;
use qubit_argument::ArgumentErrorKind;
use qubit_argument::ArgumentValue;
use qubit_argument::ComparisonConstraint;
use qubit_argument::DurationArgument;

/// Asserts that a duration failure retains its path and comparison details.
fn assert_duration_error(
    error: ArgumentError,
    expected_path: &str,
    expected_actual: Duration,
    expected_constraint: ComparisonConstraint,
) {
    assert_eq!(error.path().as_str(), expected_path);
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(expected_actual),
            constraint: expected_constraint,
        },
    );
}

/// Verifies that every successful duration validator returns the input value.
#[test]
fn test_require_methods_preserve_duration() {
    let one = Duration::from_secs(1);
    let two = Duration::from_secs(2);
    assert_eq!(one.require_positive("delay").expect("one second is positive"), one,);
    assert_eq!(
        one.require_less_than("delay", two)
            .expect("one second is less than two seconds"),
        one,
    );
    assert_eq!(
        one.require_at_most("delay", one)
            .expect("one second is at most one second"),
        one,
    );
    assert_eq!(
        two.require_greater_than("delay", one)
            .expect("two seconds is greater than one second"),
        two,
    );
    assert_eq!(
        one.require_at_least("delay", one)
            .expect("one second is at least one second"),
        one,
    );
}

/// Verifies the strict positive constraint for a zero duration.
#[test]
fn test_require_positive_reports_duration_comparison() {
    let error = Duration::ZERO
        .require_positive("keep_alive")
        .expect_err("zero keep-alive must fail");
    assert_duration_error(
        error,
        "keep_alive",
        Duration::ZERO,
        ComparisonConstraint::GreaterThan(ArgumentValue::from(Duration::ZERO)),
    );
}

/// Verifies the strict upper comparison for durations.
#[test]
fn test_require_less_than_reports_duration_comparison() {
    let value = Duration::from_secs(5);
    let error = value
        .require_less_than("delay", value)
        .expect_err("equal durations must fail a strict upper comparison");
    assert_duration_error(
        error,
        "delay",
        value,
        ComparisonConstraint::LessThan(ArgumentValue::from(value)),
    );
}

/// Verifies the inclusive upper comparison for durations.
#[test]
fn test_require_at_most_reports_duration_comparison() {
    let actual = Duration::from_secs(6);
    let bound = Duration::from_secs(5);
    let error = actual
        .require_at_most("delay", bound)
        .expect_err("six seconds exceeds five seconds");
    assert_duration_error(
        error,
        "delay",
        actual,
        ComparisonConstraint::AtMost(ArgumentValue::from(bound)),
    );
}

/// Verifies the strict lower comparison for durations.
#[test]
fn test_require_greater_than_reports_duration_comparison() {
    let value = Duration::from_secs(5);
    let error = value
        .require_greater_than("delay", value)
        .expect_err("equal durations must fail a strict lower comparison");
    assert_duration_error(
        error,
        "delay",
        value,
        ComparisonConstraint::GreaterThan(ArgumentValue::from(value)),
    );
}

/// Verifies the inclusive lower comparison for durations.
#[test]
fn test_require_at_least_reports_duration_comparison() {
    let actual = Duration::from_secs(4);
    let bound = Duration::from_secs(5);
    let error = actual
        .require_at_least("delay", bound)
        .expect_err("four seconds is below five seconds");
    assert_duration_error(
        error,
        "delay",
        actual,
        ComparisonConstraint::AtLeast(ArgumentValue::from(bound)),
    );
}
