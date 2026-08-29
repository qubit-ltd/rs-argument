// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured argument validation errors.

use qubit_argument::ArgumentBound;
use qubit_argument::ArgumentError;
use qubit_argument::ArgumentErrorKind;
use qubit_argument::ArgumentPath;
use qubit_argument::ArgumentValue;
use qubit_argument::ComparisonConstraint;
use qubit_argument::IndexRole;
use qubit_argument::LengthConstraint;
use qubit_argument::LengthMetric;
use qubit_argument::PatternExpectation;
use qubit_argument::RangeConstraint;

/// Asserts that diagnostic text contains no literal control characters.
fn assert_single_line_diagnostic(diagnostic: &str) {
    assert!(
        diagnostic.chars().all(|character| !character.is_control()),
        "diagnostic contains a literal control character: {diagnostic:?}",
    );
}

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
        metric: LengthMetric::Elements,
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
        (ArgumentErrorKind::Empty, "argument 'value' must not be empty"),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::Exact(2),
                metric: LengthMetric::Bytes,
            },
            "argument 'value' has byte length 3, expected exactly 2",
        ),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::AtLeast(4),
                metric: LengthMetric::UnicodeScalars,
            },
            "argument 'value' has Unicode scalar count 3, expected at least 4",
        ),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::AtMost(2),
                metric: LengthMetric::Elements,
            },
            "argument 'value' has element count 3, expected at most 2",
        ),
        (
            ArgumentErrorKind::Length {
                actual: 3,
                constraint: LengthConstraint::InRange { min: 4, max: 5 },
                metric: LengthMetric::Elements,
            },
            "argument 'value' has element count 3, expected between 4 and 5",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::EqualTo(ArgumentValue::from(2_i32)),
            },
            "argument 'value' has value 1, expected equal to 2",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::NotEqualTo(ArgumentValue::from(1_i32)),
            },
            "argument 'value' has value 1, expected not equal to 1",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(2_i32),
                constraint: ComparisonConstraint::LessThan(ArgumentValue::from(1_i32)),
            },
            "argument 'value' has value 2, expected less than 1",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(2_i32),
                constraint: ComparisonConstraint::AtMost(ArgumentValue::from(1_i32)),
            },
            "argument 'value' has value 2, expected at most 1",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(2_i32)),
            },
            "argument 'value' has value 1, expected greater than 2",
        ),
        (
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(1_i32),
                constraint: ComparisonConstraint::AtLeast(ArgumentValue::from(2_i32)),
            },
            "argument 'value' has value 1, expected at least 2",
        ),
        (
            ArgumentErrorKind::Range {
                actual: ArgumentValue::from(3_i32),
                constraint: RangeConstraint::new(ArgumentBound::Unbounded, ArgumentBound::Unbounded),
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
                metric: LengthMetric::Bytes,
            },
            "argument 'value' has invalid byte length constraint between 5 and 4",
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
        (ArgumentErrorKind::NotANumber, "argument 'value' contains a NaN value"),
        (
            ArgumentErrorKind::NotFinite {
                actual: ArgumentValue::from(f64::INFINITY),
            },
            "argument 'value' has non-finite value inf",
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

/// Verifies that equal numeric length data remains distinct across metrics.
#[test]
fn test_length_errors_distinguish_measurement_metrics() {
    let constraint = LengthConstraint::Exact(2);
    let bytes = ArgumentError::new(
        "value",
        ArgumentErrorKind::Length {
            actual: 1,
            constraint: constraint.clone(),
            metric: LengthMetric::Bytes,
        },
    );
    let unicode_scalars = ArgumentError::new(
        "value",
        ArgumentErrorKind::Length {
            actual: 1,
            constraint: constraint.clone(),
            metric: LengthMetric::UnicodeScalars,
        },
    );
    let elements = ArgumentError::new(
        "value",
        ArgumentErrorKind::Length {
            actual: 1,
            constraint,
            metric: LengthMetric::Elements,
        },
    );

    assert_ne!(bytes, unicode_scalars);
    assert_ne!(bytes, elements);
    assert_ne!(unicode_scalars, elements);
    assert!(matches!(
        bytes.kind(),
        ArgumentErrorKind::Length {
            metric: LengthMetric::Bytes,
            ..
        },
    ));
    assert!(matches!(
        unicode_scalars.kind(),
        ArgumentErrorKind::Length {
            metric: LengthMetric::UnicodeScalars,
            ..
        },
    ));
    assert!(matches!(
        elements.kind(),
        ArgumentErrorKind::Length {
            metric: LengthMetric::Elements,
            ..
        },
    ));
}

/// Verifies that invalid length constraints retain their measurement metric.
#[test]
fn test_invalid_length_constraint_preserves_metric() {
    let error = ArgumentError::new(
        "value",
        ArgumentErrorKind::InvalidLengthConstraint {
            constraint: LengthConstraint::InRange { min: 2, max: 1 },
            metric: LengthMetric::UnicodeScalars,
        },
    );

    assert!(matches!(
        error.kind(),
        ArgumentErrorKind::InvalidLengthConstraint {
            metric: LengthMetric::UnicodeScalars,
            ..
        },
    ));
}

/// Verifies path escaping for delimiters, backslashes, and control characters.
#[test]
fn test_display_escapes_path_on_one_line() {
    let path = "user'\\name\r\n\t\u{7}";
    let error = ArgumentError::new(path, ArgumentErrorKind::Missing);
    let diagnostic = error.to_string();

    assert_eq!(diagnostic, r"argument 'user\'\\name\r\n\t\u{7}' is missing",);
    assert_single_line_diagnostic(&diagnostic);
    assert_eq!(error.path().as_str(), path);
}

/// Verifies pattern escaping without changing the structured pattern value.
#[test]
fn test_display_escapes_pattern_on_one_line() {
    let pattern = "a'b\\c\r\n\t\u{1b}";
    let kind = ArgumentErrorKind::Pattern {
        pattern: String::from(pattern),
        expectation: PatternExpectation::Match,
    };
    let error = ArgumentError::new("value", kind.clone());
    let diagnostic = error.to_string();

    assert_eq!(diagnostic, r"argument 'value' must match pattern 'a\'b\\c\r\n\t\u{1b}'",);
    assert_single_line_diagnostic(&diagnostic);
    assert_eq!(error.kind(), &kind);
}

/// Verifies custom code and message escaping with their original values kept.
#[test]
fn test_display_escapes_custom_fields_on_one_line() {
    let code = "bad]\\code\r\n\t\u{7}";
    let message = "line\\one\r\n\t\u{1b}";
    let kind = ArgumentErrorKind::Custom {
        code: String::from(code),
        message: String::from(message),
    };
    let error = ArgumentError::new("value", kind.clone());
    let diagnostic = error.to_string();

    assert_eq!(
        diagnostic,
        r"argument 'value' failed validation [bad\]\\code\r\n\t\u{7}]: line\\one\r\n\t\u{1b}",
    );
    assert_single_line_diagnostic(&diagnostic);
    assert_eq!(error.kind(), &kind);
}
