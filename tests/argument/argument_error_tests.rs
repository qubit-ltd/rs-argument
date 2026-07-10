// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured argument validation errors.

use qubit_argument::{
    ArgumentBound,
    ArgumentError,
    ArgumentErrorKind,
    ArgumentPath,
    ArgumentValue,
    ComparisonConstraint,
    IndexRole,
    LengthConstraint,
    PatternExpectation,
    RangeConstraint,
};

/// Verifies that the final v0.4 error vocabulary is exported at the crate root.
#[test]
fn test_crate_root_exports_v04_error_vocabulary() {
    let error = ArgumentError::new("name", ArgumentErrorKind::Blank);
    let _: &ArgumentPath = error.path();
    assert_eq!(error.to_string(), "argument 'name' must not be blank");
}

/// Verifies that a structured error exposes and returns its owned components.
#[test]
fn test_argument_error_exposes_structured_parts() {
    let kind = ArgumentErrorKind::Length {
        actual: 12,
        constraint: LengthConstraint::AtMost(10),
    };
    let error = ArgumentError::new("tags", kind.clone());
    assert_eq!(error.path().as_str(), "tags");
    assert_eq!(error.kind(), &kind);
    let (path, actual_kind) = error.into_parts();
    assert_eq!(path.as_str(), "tags");
    assert_eq!(actual_kind, kind);
}

/// Verifies the standard error traits and human-readable structured display.
#[test]
fn test_argument_error_implements_standard_traits() {
    /// Asserts the standard bounds required of argument errors.
    fn assert_traits<T: std::error::Error + Send + Sync + 'static>() {}

    assert_traits::<ArgumentError>();
    let error = ArgumentError::new("name", ArgumentErrorKind::Blank);
    assert_eq!(error.to_string(), "argument 'name' must not be blank");
    assert!(format!("{error:?}").contains("Blank"));
}

/// Verifies display text for every error kind and constraint variant.
#[test]
fn test_display_formats_every_error_kind_and_constraint() {
    let cases = [
        (ArgumentErrorKind::Missing, "argument 'value' is missing"),
        (
            ArgumentErrorKind::Empty,
            "argument 'value' must not be empty",
        ),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::Exact(2),
            },
            "argument 'value' has length 3, expected exactly 2",
        ),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::AtLeast(4),
            },
            "argument 'value' has length 3, expected at least 4",
        ),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::AtMost(2),
            },
            "argument 'value' has length 3, expected at most 2",
        ),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::InRange { min: 4, max: 5 },
            },
            "argument 'value' has length 3, expected between 4 and 5",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::EqualTo(ArgumentValue::from(
                    2_i32,
                )),
            },
            "argument 'value' has value 1, expected equal to 2",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::NotEqualTo(
                    ArgumentValue::from(1_i32),
                ),
            },
            "argument 'value' has value 1, expected not equal to 1",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(2_i32),
                constraint: ComparisonConstraint::LessThan(
                    ArgumentValue::from(1_i32),
                ),
            },
            "argument 'value' has value 2, expected less than 1",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(2_i32),
                constraint: ComparisonConstraint::AtMost(ArgumentValue::from(
                    1_i32,
                )),
            },
            "argument 'value' has value 2, expected at most 1",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::GreaterThan(
                    ArgumentValue::from(2_i32),
                ),
            },
            "argument 'value' has value 1, expected greater than 2",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::AtLeast(ArgumentValue::from(
                    2_i32,
                )),
            },
            "argument 'value' has value 1, expected at least 2",
        ),
        (
            ArgumentErrorKind::Range {
                actual: ArgumentValue::from(3_i32),
                constraint: RangeConstraint::new(
                    ArgumentBound::Unbounded,
                    ArgumentBound::Unbounded,
                ),
            },
            "argument 'value' has value 3, expected range (-infinity, infinity)",
        ),
        (
            ArgumentErrorKind::Range {
                actual: ArgumentValue::from(3_i32),
                constraint: RangeConstraint::new(
                    ArgumentBound::Included(ArgumentValue::from(1_i32)),
                    ArgumentBound::Included(ArgumentValue::from(2_i32)),
                ),
            },
            "argument 'value' has value 3, expected range [1, 2]",
        ),
        (
            ArgumentErrorKind::Range {
                actual: ArgumentValue::from(3_i32),
                constraint: RangeConstraint::new(
                    ArgumentBound::Excluded(ArgumentValue::from(1_i32)),
                    ArgumentBound::Excluded(ArgumentValue::from(2_i32)),
                ),
            },
            "argument 'value' has value 3, expected range (1, 2)",
        ),
        (
            ArgumentErrorKind::InvalidLengthConstraint {
                constraint: LengthConstraint::InRange { min: 5, max: 4 },
            },
            "argument 'value' has invalid length constraint between 5 and 4",
        ),
        (
            ArgumentErrorKind::InvalidRangeConstraint {
                constraint: RangeConstraint::new(
                    ArgumentBound::Included(ArgumentValue::from(2_i32)),
                    ArgumentBound::Excluded(ArgumentValue::from(1_i32)),
                ),
            },
            "argument 'value' has invalid range constraint [2, 1)",
        ),
        (
            ArgumentErrorKind::NotANumber,
            "argument 'value' must be a number",
        ),
        (
            ArgumentErrorKind::Index {
                index: 3,
                size: 3,
                role: IndexRole::Element,
            },
            "argument 'value' has element index 3 outside the valid range 0..3",
        ),
        (
            ArgumentErrorKind::Index {
                index: 4,
                size: 3,
                role: IndexRole::Position,
            },
            "argument 'value' has position index 4 outside the valid range 0..=3",
        ),
        (
            ArgumentErrorKind::IndexRange {
                start: 3,
                end: 2,
                size: 4,
            },
            "argument 'value' has position range 3..2 outside the valid range 0..=4",
        ),
        (
            ArgumentErrorKind::Bounds {
                offset: 4,
                length: 2,
                total_length: 5,
            },
            "argument 'value' has offset 4 and length 2 outside total length 5",
        ),
        (
            ArgumentErrorKind::Pattern {
                pattern: String::from("^[a-z]+$"),
                expectation: PatternExpectation::Match,
            },
            "argument 'value' must match pattern '^[a-z]+$'",
        ),
        (
            ArgumentErrorKind::Pattern {
                pattern: String::from("^[0-9]+$"),
                expectation: PatternExpectation::NoMatch,
            },
            "argument 'value' must not match pattern '^[0-9]+$'",
        ),
        (
            ArgumentErrorKind::Custom {
                code: String::from("odd"),
                message: String::from("must be even"),
            },
            "argument 'value' failed validation [odd]: must be even",
        ),
    ];

    for (kind, expected) in cases {
        assert_eq!(ArgumentError::new("value", kind).to_string(), expected);
    }
}
