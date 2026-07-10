// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured errors produced by argument validation.

use std::fmt::{
    self,
    Display,
    Formatter,
};

use crate::argument::{
    ArgumentBound,
    ArgumentErrorKind,
    ArgumentPath,
    ComparisonConstraint,
    IndexRole,
    LengthConstraint,
    LengthMetric,
    PatternExpectation,
    RangeConstraint,
};

/// A structured argument validation failure.
///
/// The error owns its argument path and failure kind, allowing downstream
/// error types to inspect or preserve it without parsing display text.
/// [`Display`] escapes caller-provided fields into a single-line diagnostic;
/// accessors and [`Debug`] continue to expose the original structured values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentError {
    path: ArgumentPath,
    kind: Box<ArgumentErrorKind>,
}

impl ArgumentError {
    /// Creates an error from an argument path and structured failure kind.
    ///
    /// `path` is copied only while constructing the error. The supplied
    /// `kind` is retained unchanged.
    #[inline]
    pub fn new(path: &str, kind: ArgumentErrorKind) -> Self {
        Self {
            path: ArgumentPath::new(path),
            kind: Box::new(kind),
        }
    }

    /// Returns the path of the argument that failed validation.
    #[inline]
    pub fn path(&self) -> &ArgumentPath {
        &self.path
    }

    /// Returns the structured validation failure kind.
    #[inline]
    pub fn kind(&self) -> &ArgumentErrorKind {
        self.kind.as_ref()
    }

    /// Consumes the error and returns its owned path and failure kind.
    ///
    /// The first tuple element is the argument path and the second is the
    /// structured failure kind.
    #[inline]
    pub fn into_parts(self) -> (ArgumentPath, ArgumentErrorKind) {
        let Self { path, kind } = self;
        (path, *kind)
    }
}

impl Display for ArgumentError {
    /// Formats a single-line diagnostic entirely from the structured fields.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let path = escape_for_display(self.path.as_ref(), Some('\''));
        write!(formatter, "argument '{path}'")?;
        match self.kind.as_ref() {
            ArgumentErrorKind::Missing => formatter.write_str(" is missing"),
            ArgumentErrorKind::Blank => {
                formatter.write_str(" must not be blank")
            }
            ArgumentErrorKind::Empty => {
                formatter.write_str(" must not be empty")
            }
            ArgumentErrorKind::Length {
                actual,
                constraint,
                metric,
            } => {
                let label = length_metric_label(metric);
                write!(formatter, " has {label} {actual}, expected ")?;
                write_length_constraint(formatter, constraint)
            }
            ArgumentErrorKind::Comparison { actual, constraint } => {
                write!(formatter, " has value {actual}, expected ")?;
                write_comparison_constraint(formatter, constraint)
            }
            ArgumentErrorKind::Range { actual, constraint } => {
                write!(formatter, " has value {actual}, expected range ")?;
                write_range_constraint(formatter, constraint)
            }
            ArgumentErrorKind::InvalidLengthConstraint {
                constraint,
                metric,
            } => {
                let label = length_metric_label(metric);
                write!(formatter, " has invalid {label} constraint ")?;
                write_length_constraint(formatter, constraint)
            }
            ArgumentErrorKind::InvalidRangeConstraint { constraint } => {
                formatter.write_str(" has invalid range constraint ")?;
                write_range_constraint(formatter, constraint)
            }
            ArgumentErrorKind::NotANumber => {
                formatter.write_str(" must be a number")
            }
            ArgumentErrorKind::Index { index, size, role } => match role {
                IndexRole::Element => write!(
                    formatter,
                    " has element index {index} outside the valid range 0..{size}",
                ),
                IndexRole::Position => write!(
                    formatter,
                    " has position index {index} outside the valid range 0..={size}",
                ),
            },
            ArgumentErrorKind::IndexRange { start, end, size } => write!(
                formatter,
                " has position range {start}..{end} outside the valid range 0..={size}",
            ),
            ArgumentErrorKind::Bounds {
                offset,
                length,
                total_length,
            } => write!(
                formatter,
                " has offset {offset} and length {length} outside total length {total_length}",
            ),
            ArgumentErrorKind::Pattern {
                pattern,
                expectation,
            } => match expectation {
                PatternExpectation::Match => {
                    let pattern = escape_for_display(pattern, Some('\''));
                    write!(formatter, " must match pattern '{pattern}'")
                }
                PatternExpectation::NoMatch => {
                    let pattern = escape_for_display(pattern, Some('\''));
                    write!(formatter, " must not match pattern '{pattern}'")
                }
            },
            ArgumentErrorKind::Custom { code, message } => {
                let code = escape_for_display(code, Some(']'));
                let message = escape_for_display(message, None);
                write!(formatter, " failed validation [{code}]: {message}")
            }
        }
    }
}

impl std::error::Error for ArgumentError {}

/// Escapes caller-provided text for a single-line diagnostic.
///
/// The returned string represents `value` with backslashes, carriage returns,
/// line feeds, tabs, other control characters, and the optional active
/// `delimiter` escaped. The input remains unchanged.
fn escape_for_display(value: &str, delimiter: Option<char>) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '\r' => escaped.push_str("\\r"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            character if Some(character) == delimiter => {
                escaped.push('\\');
                escaped.push(character);
            }
            character if character.is_control() => {
                escaped.extend(character.escape_unicode());
            }
            character => escaped.push(character),
        }
    }
    escaped
}

/// Returns the human-readable unit label for a measured length.
///
/// `metric` determines whether diagnostics describe UTF-8 byte length,
/// Unicode scalar count, or collection element count.
fn length_metric_label(metric: &LengthMetric) -> &'static str {
    match metric {
        LengthMetric::Bytes => "byte length",
        LengthMetric::UnicodeScalars => "Unicode scalar count",
        LengthMetric::Elements => "element count",
    }
}

/// Writes a length constraint in human-readable form.
///
/// `formatter` receives only text derived from `constraint`. Formatting errors
/// from the destination are returned unchanged.
fn write_length_constraint(
    formatter: &mut Formatter<'_>,
    constraint: &LengthConstraint,
) -> fmt::Result {
    match constraint {
        LengthConstraint::Exact(expected) => {
            write!(formatter, "exactly {expected}")
        }
        LengthConstraint::AtLeast(min) => write!(formatter, "at least {min}"),
        LengthConstraint::AtMost(max) => write!(formatter, "at most {max}"),
        LengthConstraint::InRange { min, max } => {
            write!(formatter, "between {min} and {max}")
        }
    }
}

/// Writes a numeric comparison constraint in human-readable form.
///
/// `formatter` receives only text derived from `constraint`. Formatting errors
/// from the destination are returned unchanged.
fn write_comparison_constraint(
    formatter: &mut Formatter<'_>,
    constraint: &ComparisonConstraint,
) -> fmt::Result {
    match constraint {
        ComparisonConstraint::EqualTo(expected) => {
            write!(formatter, "equal to {expected}")
        }
        ComparisonConstraint::NotEqualTo(expected) => {
            write!(formatter, "not equal to {expected}")
        }
        ComparisonConstraint::LessThan(bound) => {
            write!(formatter, "less than {bound}")
        }
        ComparisonConstraint::AtMost(bound) => {
            write!(formatter, "at most {bound}")
        }
        ComparisonConstraint::GreaterThan(bound) => {
            write!(formatter, "greater than {bound}")
        }
        ComparisonConstraint::AtLeast(bound) => {
            write!(formatter, "at least {bound}")
        }
    }
}

/// Writes a numeric range with notation that preserves both bound kinds.
///
/// `formatter` receives only text derived from `constraint`. Formatting errors
/// from the destination are returned unchanged.
fn write_range_constraint(
    formatter: &mut Formatter<'_>,
    constraint: &RangeConstraint,
) -> fmt::Result {
    match constraint.lower() {
        ArgumentBound::Unbounded => formatter.write_str("(-infinity"),
        ArgumentBound::Included(value) => write!(formatter, "[{value}"),
        ArgumentBound::Excluded(value) => write!(formatter, "({value}"),
    }?;
    formatter.write_str(", ")?;
    match constraint.upper() {
        ArgumentBound::Unbounded => formatter.write_str("infinity)"),
        ArgumentBound::Included(value) => write!(formatter, "{value}]"),
        ArgumentBound::Excluded(value) => write!(formatter, "{value})"),
    }
}

/// Result type returned by argument validation operations.
pub type ArgumentResult<T> = Result<T, ArgumentError>;
