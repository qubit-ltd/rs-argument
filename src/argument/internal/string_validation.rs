// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String blankness, length, and optional pattern validation.

#[cfg(feature = "regex")]
use regex::Regex;

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::LengthConstraint;
use crate::argument::LengthMetric;
#[cfg(feature = "regex")]
use crate::argument::PatternExpectation;

/// Validates that `value` contains a non-whitespace Unicode scalar value.
///
/// `value` is inspected without allocation. The function returns `Ok(())`
/// when at least one scalar value is not whitespace; otherwise, it returns
/// [`ArgumentErrorKind::Blank`] at `path` without storing `value`.
///
/// # Parameters
///
/// - `value`: The string to inspect without retaining it.
/// - `path`: Identifies the argument in a validation error.
///
/// # Returns
///
/// `Ok(())` when at least one Unicode scalar is not whitespace.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::Blank`] for an empty or whitespace-only string.
pub(in crate::argument) fn validate_non_blank(value: &str, path: &str) -> ArgumentResult<()> {
    if value.chars().all(char::is_whitespace) {
        Err(ArgumentError::new(path, ArgumentErrorKind::Blank))
    } else {
        Ok(())
    }
}

/// Validates an observed length against a structured length constraint.
///
/// `actual` is compared with `constraint`, `metric` records how it was
/// measured, and `path` identifies any failure. A reversed inclusive range
/// returns
/// [`ArgumentErrorKind::InvalidLengthConstraint`] before `actual` is checked.
/// Any other unsatisfied constraint returns [`ArgumentErrorKind::Length`]; a
/// satisfied constraint returns `Ok(())`.
///
/// # Parameters
///
/// - `path`: Identifies the argument in a validation error.
/// - `actual`: The observed length in units of `metric`.
/// - `constraint`: The required relationship, moved into an error on failure.
/// - `metric`: The measurement unit retained in a validation error.
///
/// # Returns
///
/// `Ok(())` when the constraint is valid and satisfied.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::InvalidLengthConstraint`] for a reversed range,
/// or [`ArgumentErrorKind::Length`] when the observed length fails the
/// constraint.
pub(in crate::argument) fn validate_length(
    path: &str,
    actual: usize,
    constraint: LengthConstraint,
    metric: LengthMetric,
) -> ArgumentResult<()> {
    if let LengthConstraint::InRange { min, max } = &constraint
        && min > max
    {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::InvalidLengthConstraint { constraint, metric },
        ));
    }

    let is_valid = match &constraint {
        LengthConstraint::Exact(expected) => actual == *expected,
        LengthConstraint::AtLeast(min) => actual >= *min,
        LengthConstraint::AtMost(max) => actual <= *max,
        LengthConstraint::InRange { min, max } => actual >= *min && actual <= *max,
    };
    if is_valid {
        Ok(())
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Length {
                actual,
                constraint,
                metric,
            },
        ))
    }
}

/// Validates one regex expectation without retaining the inspected string.
///
/// `value` is tested with `pattern` according to `expectation`, and `path`
/// identifies any failure. Success returns `Ok(())`; failure returns
/// [`ArgumentErrorKind::Pattern`] containing only the pattern text and
/// expectation, never `value`.
///
/// # Parameters
///
/// - `value`: The string to inspect without retaining it.
/// - `path`: Identifies the argument in a validation error.
/// - `pattern`: The compiled regex, used without implicit anchoring.
/// - `expectation`: Whether a match or a non-match is required.
///
/// # Returns
///
/// `Ok(())` when the match result satisfies `expectation`.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::Pattern`] when the expectation is not
/// satisfied.
#[cfg(feature = "regex")]
pub(in crate::argument) fn validate_pattern(
    value: &str,
    path: &str,
    pattern: &Regex,
    expectation: PatternExpectation,
) -> ArgumentResult<()> {
    let matches = pattern.is_match(value);
    let is_valid = match expectation {
        PatternExpectation::Match => matches,
        PatternExpectation::NoMatch => !matches,
    };
    if is_valid {
        Ok(())
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Pattern {
                pattern: String::from(pattern.as_str()),
                expectation,
            },
        ))
    }
}
