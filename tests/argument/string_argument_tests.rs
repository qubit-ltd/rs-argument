// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for ownership-preserving string argument validation.

use qubit_argument::{ArgumentError, ArgumentErrorKind, LengthConstraint, StringArgument};

#[cfg(feature = "regex")]
use qubit_argument::PatternExpectation;
#[cfg(feature = "regex")]
use regex::Regex;

/// Asserts that an error has the requested path and structured kind.
fn assert_structured_error(
    error: ArgumentError,
    expected_path: &str,
    expected_kind: ArgumentErrorKind,
) {
    assert_eq!(error.path().as_str(), expected_path);
    assert_eq!(error.kind(), &expected_kind);
}

/// Verifies that non-blank validation returns an owned string unchanged.
#[test]
fn test_require_non_blank_preserves_owned_string() {
    let value = String::from("qubit");
    let validated: String = value.require_non_blank("name").expect("name is non-blank");
    assert_eq!(validated, "qubit");
}

/// Verifies that non-blank validation preserves a borrowed string.
#[test]
fn test_require_non_blank_preserves_borrowed_str() {
    let value: &str = "qubit";
    let validated: &str = value.require_non_blank("name").expect("name is non-blank");
    assert_eq!(validated, value);
}

/// Verifies that empty and Unicode-whitespace strings are blank.
#[test]
fn test_require_non_blank_rejects_unicode_whitespace() {
    for value in ["", "\u{2003}\u{3000}", " \t\n"] {
        let error = value
            .require_non_blank("name")
            .expect_err("whitespace-only input must be blank");
        assert_structured_error(error, "name", ArgumentErrorKind::Blank);
    }
}

/// Verifies that byte length and Unicode scalar count are distinct.
#[test]
fn test_byte_len_and_char_count_have_distinct_semantics() {
    let value = "汉😀";
    assert!(value.require_byte_len("value", 7).is_ok());
    assert!(value.require_char_count("value", 2).is_ok());
}

/// Verifies the exact byte-length constraint and its structured error.
#[test]
fn test_require_byte_len_returns_exact_length_error() {
    assert_eq!(
        "abc"
            .require_byte_len("value", 3)
            .expect("three ASCII bytes satisfy the constraint"),
        "abc",
    );
    let error = "ab"
        .require_byte_len("value", 3)
        .expect_err("two bytes do not satisfy an exact length of three");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Length {
            actual: 2,
            constraint: LengthConstraint::Exact(3),
        },
    );
}

/// Verifies the minimum byte-length constraint and its structured error.
#[test]
fn test_require_byte_len_at_least_returns_minimum_length_error() {
    assert_eq!(
        "abc"
            .require_byte_len_at_least("value", 3)
            .expect("three bytes satisfy a minimum of three"),
        "abc",
    );
    let error = "ab"
        .require_byte_len_at_least("value", 3)
        .expect_err("two bytes are below the minimum");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Length {
            actual: 2,
            constraint: LengthConstraint::AtLeast(3),
        },
    );
}

/// Verifies the maximum byte-length constraint and its structured error.
#[test]
fn test_require_byte_len_at_most_returns_maximum_length_error() {
    assert_eq!(
        "abc"
            .require_byte_len_at_most("value", 3)
            .expect("three bytes satisfy a maximum of three"),
        "abc",
    );
    let error = "abcd"
        .require_byte_len_at_most("value", 3)
        .expect_err("four bytes exceed the maximum");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Length {
            actual: 4,
            constraint: LengthConstraint::AtMost(3),
        },
    );
}

/// Verifies both inclusive limits of a byte-length range.
#[test]
fn test_require_byte_len_in_returns_range_length_errors() {
    assert_eq!(
        "abc"
            .require_byte_len_in("value", 3, 4)
            .expect("three bytes lie on the inclusive lower boundary"),
        "abc",
    );
    for value in ["ab", "abcde"] {
        let error = value
            .require_byte_len_in("value", 3, 4)
            .expect_err("byte length lies outside the range");
        assert_structured_error(
            error,
            "value",
            ArgumentErrorKind::Length {
                actual: value.len(),
                constraint: LengthConstraint::InRange { min: 3, max: 4 },
            },
        );
    }
}

/// Verifies that a reversed byte-length range fails before value validation.
#[test]
fn test_require_byte_len_in_rejects_invalid_constraint_first() {
    let error = "x"
        .require_byte_len_in("value", 3, 1)
        .expect_err("a reversed byte-length range is invalid");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::InvalidLengthConstraint {
            constraint: LengthConstraint::InRange { min: 3, max: 1 },
        },
    );
}

/// Verifies the exact Unicode scalar-count constraint and its error.
#[test]
fn test_require_char_count_returns_exact_length_error() {
    assert_eq!(
        "汉😀"
            .require_char_count("value", 2)
            .expect("the input has two Unicode scalar values"),
        "汉😀",
    );
    let error = "汉😀"
        .require_char_count("value", 3)
        .expect_err("two scalar values do not satisfy a count of three");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Length {
            actual: 2,
            constraint: LengthConstraint::Exact(3),
        },
    );
}

/// Verifies the minimum Unicode scalar-count constraint and its error.
#[test]
fn test_require_char_count_at_least_returns_minimum_length_error() {
    assert_eq!(
        "汉😀"
            .require_char_count_at_least("value", 2)
            .expect("two scalar values satisfy a minimum of two"),
        "汉😀",
    );
    let error = "汉"
        .require_char_count_at_least("value", 2)
        .expect_err("one scalar value is below the minimum");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Length {
            actual: 1,
            constraint: LengthConstraint::AtLeast(2),
        },
    );
}

/// Verifies the maximum Unicode scalar-count constraint and its error.
#[test]
fn test_require_char_count_at_most_returns_maximum_length_error() {
    assert_eq!(
        "汉😀"
            .require_char_count_at_most("value", 2)
            .expect("two scalar values satisfy a maximum of two"),
        "汉😀",
    );
    let error = "汉😀x"
        .require_char_count_at_most("value", 2)
        .expect_err("three scalar values exceed the maximum");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::Length {
            actual: 3,
            constraint: LengthConstraint::AtMost(2),
        },
    );
}

/// Verifies both inclusive limits of a Unicode scalar-count range.
#[test]
fn test_require_char_count_in_returns_range_length_errors() {
    assert_eq!(
        "汉😀"
            .require_char_count_in("value", 1, 2)
            .expect("two scalar values lie on the inclusive upper boundary"),
        "汉😀",
    );
    for value in ["", "汉😀x"] {
        let error = value
            .require_char_count_in("value", 1, 2)
            .expect_err("scalar count lies outside the range");
        assert_structured_error(
            error,
            "value",
            ArgumentErrorKind::Length {
                actual: value.chars().count(),
                constraint: LengthConstraint::InRange { min: 1, max: 2 },
            },
        );
    }
}

/// Verifies that a reversed scalar-count range fails before value validation.
#[test]
fn test_require_char_count_in_rejects_invalid_constraint_first() {
    let error = "x"
        .require_char_count_in("value", 3, 1)
        .expect_err("a reversed scalar-count range is invalid");
    assert_structured_error(
        error,
        "value",
        ArgumentErrorKind::InvalidLengthConstraint {
            constraint: LengthConstraint::InRange { min: 3, max: 1 },
        },
    );
}

/// Verifies that empty strings have zero byte and scalar lengths.
#[test]
fn test_length_methods_accept_empty_string_at_zero() {
    assert_eq!(
        "".require_byte_len("value", 0)
            .expect("the empty string has zero bytes"),
        "",
    );
    assert_eq!(
        "".require_char_count("value", 0)
            .expect("the empty string has zero scalar values"),
        "",
    );
}

/// Verifies that all core validators can return the same owned string.
#[test]
fn test_core_string_validation_chain_preserves_ownership() {
    let validated: String = String::from("汉😀")
        .require_non_blank("value")
        .and_then(|value| value.require_byte_len("value", 7))
        .and_then(|value| value.require_byte_len_at_least("value", 7))
        .and_then(|value| value.require_byte_len_at_most("value", 7))
        .and_then(|value| value.require_byte_len_in("value", 7, 7))
        .and_then(|value| value.require_char_count("value", 2))
        .and_then(|value| value.require_char_count_at_least("value", 2))
        .and_then(|value| value.require_char_count_at_most("value", 2))
        .and_then(|value| value.require_char_count_in("value", 2, 2))
        .expect("every constraint accepts the owned value");
    assert_eq!(validated, "汉😀");
}

/// Verifies that string errors do not expose the validated input.
#[test]
fn test_string_error_does_not_expose_input() {
    let secret = "secret-token-value";
    let error = secret
        .require_byte_len_at_most("token", 4)
        .expect_err("secret is too long");
    assert!(!format!("{error:?}").contains(secret));
    assert!(!error.to_string().contains(secret));
}

/// Verifies the pattern error emitted when a match is required.
#[cfg(feature = "regex")]
#[test]
fn test_require_match_returns_pattern_error_without_input() {
    let pattern = Regex::new("^[a-z]+$").expect("test pattern is valid");
    let secret = "123-secret-token";
    let error = secret
        .require_match("name", &pattern)
        .expect_err("digits and punctuation must fail");
    assert_structured_error(
        error.clone(),
        "name",
        ArgumentErrorKind::Pattern {
            pattern: String::from("^[a-z]+$"),
            expectation: PatternExpectation::Match,
        },
    );
    assert!(!format!("{error:?}").contains(secret));
    assert!(!error.to_string().contains(secret));
}

/// Verifies the pattern error emitted when a non-match is required.
#[cfg(feature = "regex")]
#[test]
fn test_require_not_match_returns_pattern_error() {
    let pattern = Regex::new("^[0-9]+$").expect("test pattern is valid");
    let error = "123"
        .require_not_match("identifier", &pattern)
        .expect_err("digits match the forbidden pattern");
    assert_structured_error(
        error,
        "identifier",
        ArgumentErrorKind::Pattern {
            pattern: String::from("^[0-9]+$"),
            expectation: PatternExpectation::NoMatch,
        },
    );
}

/// Verifies that regex validation preserves an owned string on success.
#[cfg(feature = "regex")]
#[test]
fn test_require_match_preserves_owned_string() {
    let pattern = Regex::new("^[a-z]+$").expect("test pattern is valid");
    let validated: String = String::from("qubit")
        .require_match("name", &pattern)
        .and_then(|value| {
            value.require_not_match(
                "name",
                &Regex::new("^[0-9]+$").expect("test pattern is valid"),
            )
        })
        .expect("the owned string satisfies both pattern constraints");
    assert_eq!(validated, "qubit");
}
