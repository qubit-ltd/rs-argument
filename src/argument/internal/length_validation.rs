// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared validation for measured argument lengths.

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::LengthConstraint;
use crate::argument::LengthMetric;

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
