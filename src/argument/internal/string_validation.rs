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
