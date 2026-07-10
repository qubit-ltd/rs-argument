// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for string arguments.

use crate::argument::{
    ArgumentError,
    ArgumentErrorKind,
    ArgumentResult,
    LengthConstraint,
    LengthMetric,
};

#[cfg(feature = "regex")]
use crate::argument::PatternExpectation;
#[cfg(feature = "regex")]
use regex::Regex;

/// Validates string arguments while preserving ownership or borrowing.
///
/// Byte-length methods count UTF-8 bytes, while character-count methods count
/// Unicode scalar values. Every successful method returns the original value
/// without cloning it. String contents are inspected but never captured in an
/// error. Length failures use [`LengthMetric::Bytes`] for byte methods and
/// [`LengthMetric::UnicodeScalars`] for character-count methods.
pub trait StringArgument: Sized {
    /// Requires this string to contain at least one non-whitespace character.
    ///
    /// Success returns the original value without cloning. Empty strings and
    /// strings whose Unicode scalar values are all whitespace return
    /// [`ArgumentErrorKind::Blank`] at `path`.
    fn require_non_blank(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this string to contain exactly `expected` UTF-8 bytes.
    ///
    /// Success returns the original value without cloning. A different byte
    /// length returns [`ArgumentErrorKind::Length`] at `path`.
    fn require_byte_len(
        self,
        path: &str,
        expected: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string to contain at least `min` UTF-8 bytes.
    ///
    /// Success returns the original value without cloning. A smaller byte
    /// length returns [`ArgumentErrorKind::Length`] at `path`.
    fn require_byte_len_at_least(
        self,
        path: &str,
        min: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string to contain at most `max` UTF-8 bytes.
    ///
    /// Success returns the original value without cloning. A larger byte
    /// length returns [`ArgumentErrorKind::Length`] at `path`.
    fn require_byte_len_at_most(
        self,
        path: &str,
        max: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string's UTF-8 byte length to lie in `min..=max`.
    ///
    /// The range is validated before the string length. If `min > max`, this
    /// returns [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`;
    /// otherwise, an out-of-range length returns
    /// [`ArgumentErrorKind::Length`]. Success returns the original value
    /// without cloning.
    fn require_byte_len_in(
        self,
        path: &str,
        min: usize,
        max: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string to contain exactly `expected` Unicode scalar
    /// values.
    ///
    /// Success returns the original value without cloning. A different scalar
    /// count returns [`ArgumentErrorKind::Length`] at `path`.
    fn require_char_count(
        self,
        path: &str,
        expected: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string to contain at least `min` Unicode scalar values.
    ///
    /// Success returns the original value without cloning. A smaller scalar
    /// count returns [`ArgumentErrorKind::Length`] at `path`.
    fn require_char_count_at_least(
        self,
        path: &str,
        min: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string to contain at most `max` Unicode scalar values.
    ///
    /// Success returns the original value without cloning. A larger scalar
    /// count returns [`ArgumentErrorKind::Length`] at `path`.
    fn require_char_count_at_most(
        self,
        path: &str,
        max: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string's Unicode scalar count to lie in `min..=max`.
    ///
    /// The range is validated before the character count. If `min > max`, this
    /// returns [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`;
    /// otherwise, an out-of-range count returns [`ArgumentErrorKind::Length`].
    /// Success returns the original value without cloning.
    fn require_char_count_in(
        self,
        path: &str,
        min: usize,
        max: usize,
    ) -> ArgumentResult<Self>;

    /// Requires this string to match `pattern`.
    ///
    /// Matching uses [`Regex::is_match`] without implicit anchoring. Success
    /// returns the original value without cloning; failure returns
    /// [`ArgumentErrorKind::Pattern`] at `path` without capturing the input.
    #[cfg(feature = "regex")]
    fn require_match(self, path: &str, pattern: &Regex)
    -> ArgumentResult<Self>;

    /// Requires this string not to match `pattern`.
    ///
    /// Matching uses [`Regex::is_match`] without implicit anchoring. Success
    /// returns the original value without cloning; failure returns
    /// [`ArgumentErrorKind::Pattern`] at `path` without capturing the input.
    #[cfg(feature = "regex")]
    fn require_not_match(
        self,
        path: &str,
        pattern: &Regex,
    ) -> ArgumentResult<Self>;
}

impl StringArgument for &str {
    /// Validates Unicode blankness and returns the original borrow.
    ///
    /// `path` identifies a [`ArgumentErrorKind::Blank`] failure.
    #[inline]
    fn require_non_blank(self, path: &str) -> ArgumentResult<Self> {
        validate_non_blank(self, path)?;
        Ok(self)
    }

    /// Validates the exact UTF-8 byte length and returns the original borrow.
    ///
    /// A mismatch returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_byte_len(
        self,
        path: &str,
        expected: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::Exact(expected),
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates the minimum UTF-8 byte length and returns the original borrow.
    ///
    /// A value below `min` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_byte_len_at_least(
        self,
        path: &str,
        min: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::AtLeast(min),
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates the maximum UTF-8 byte length and returns the original borrow.
    ///
    /// A value above `max` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_byte_len_at_most(
        self,
        path: &str,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::AtMost(max),
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates an inclusive UTF-8 byte-length range and returns the borrow.
    ///
    /// `min > max` returns [`ArgumentErrorKind::InvalidLengthConstraint`] at
    /// `path`; an out-of-range value returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_byte_len_in(
        self,
        path: &str,
        min: usize,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::InRange { min, max },
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates the exact Unicode scalar count and returns the original
    /// borrow.
    ///
    /// A mismatch returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_char_count(
        self,
        path: &str,
        expected: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::Exact(expected),
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates the minimum Unicode scalar count and returns the original
    /// borrow.
    ///
    /// A count below `min` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_char_count_at_least(
        self,
        path: &str,
        min: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::AtLeast(min),
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates the maximum Unicode scalar count and returns the original
    /// borrow.
    ///
    /// A count above `max` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_char_count_at_most(
        self,
        path: &str,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::AtMost(max),
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates an inclusive Unicode scalar-count range and returns the
    /// borrow.
    ///
    /// `min > max` returns [`ArgumentErrorKind::InvalidLengthConstraint`] at
    /// `path`; an out-of-range count returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_char_count_in(
        self,
        path: &str,
        min: usize,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::InRange { min, max },
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates a required regex match and returns the original borrow.
    ///
    /// A non-match returns [`ArgumentErrorKind::Pattern`] at `path` without
    /// capturing the input string.
    #[cfg(feature = "regex")]
    #[inline]
    fn require_match(
        self,
        path: &str,
        pattern: &Regex,
    ) -> ArgumentResult<Self> {
        validate_pattern(self, path, pattern, PatternExpectation::Match)?;
        Ok(self)
    }

    /// Validates a required regex non-match and returns the original borrow.
    ///
    /// A match returns [`ArgumentErrorKind::Pattern`] at `path` without
    /// capturing the input string.
    #[cfg(feature = "regex")]
    #[inline]
    fn require_not_match(
        self,
        path: &str,
        pattern: &Regex,
    ) -> ArgumentResult<Self> {
        validate_pattern(self, path, pattern, PatternExpectation::NoMatch)?;
        Ok(self)
    }
}

impl StringArgument for String {
    /// Validates Unicode blankness and returns the original owned string.
    ///
    /// `path` identifies a [`ArgumentErrorKind::Blank`] failure.
    #[inline]
    fn require_non_blank(self, path: &str) -> ArgumentResult<Self> {
        validate_non_blank(self.as_str(), path)?;
        Ok(self)
    }

    /// Validates the exact UTF-8 byte length and returns the owned string.
    ///
    /// A mismatch returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_byte_len(
        self,
        path: &str,
        expected: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::Exact(expected),
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates the minimum UTF-8 byte length and returns the owned string.
    ///
    /// A value below `min` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_byte_len_at_least(
        self,
        path: &str,
        min: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::AtLeast(min),
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates the maximum UTF-8 byte length and returns the owned string.
    ///
    /// A value above `max` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_byte_len_at_most(
        self,
        path: &str,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::AtMost(max),
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates an inclusive UTF-8 byte-length range and returns the string.
    ///
    /// `min > max` returns [`ArgumentErrorKind::InvalidLengthConstraint`] at
    /// `path`; an out-of-range value returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_byte_len_in(
        self,
        path: &str,
        min: usize,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::InRange { min, max },
            LengthMetric::Bytes,
        )?;
        Ok(self)
    }

    /// Validates the exact Unicode scalar count and returns the owned string.
    ///
    /// A mismatch returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_char_count(
        self,
        path: &str,
        expected: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::Exact(expected),
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates the minimum Unicode scalar count and returns the owned string.
    ///
    /// A count below `min` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_char_count_at_least(
        self,
        path: &str,
        min: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::AtLeast(min),
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates the maximum Unicode scalar count and returns the owned string.
    ///
    /// A count above `max` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_char_count_at_most(
        self,
        path: &str,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::AtMost(max),
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates an inclusive Unicode scalar-count range and returns the
    /// string.
    ///
    /// `min > max` returns [`ArgumentErrorKind::InvalidLengthConstraint`] at
    /// `path`; an out-of-range count returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_char_count_in(
        self,
        path: &str,
        min: usize,
        max: usize,
    ) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.chars().count(),
            LengthConstraint::InRange { min, max },
            LengthMetric::UnicodeScalars,
        )?;
        Ok(self)
    }

    /// Validates a required regex match and returns the owned string.
    ///
    /// A non-match returns [`ArgumentErrorKind::Pattern`] at `path` without
    /// capturing the input string.
    #[cfg(feature = "regex")]
    #[inline]
    fn require_match(
        self,
        path: &str,
        pattern: &Regex,
    ) -> ArgumentResult<Self> {
        validate_pattern(
            self.as_str(),
            path,
            pattern,
            PatternExpectation::Match,
        )?;
        Ok(self)
    }

    /// Validates a required regex non-match and returns the owned string.
    ///
    /// A match returns [`ArgumentErrorKind::Pattern`] at `path` without
    /// capturing the input string.
    #[cfg(feature = "regex")]
    #[inline]
    fn require_not_match(
        self,
        path: &str,
        pattern: &Regex,
    ) -> ArgumentResult<Self> {
        validate_pattern(
            self.as_str(),
            path,
            pattern,
            PatternExpectation::NoMatch,
        )?;
        Ok(self)
    }
}

/// Validates that `value` contains a non-whitespace Unicode scalar value.
///
/// `value` is inspected without allocation. The function returns `Ok(())`
/// when at least one scalar value is not whitespace; otherwise, it returns
/// [`ArgumentErrorKind::Blank`] at `path` without storing `value`.
fn validate_non_blank(value: &str, path: &str) -> ArgumentResult<()> {
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
fn validate_length(
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
        LengthConstraint::InRange { min, max } => {
            actual >= *min && actual <= *max
        }
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
#[cfg(feature = "regex")]
fn validate_pattern(
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
