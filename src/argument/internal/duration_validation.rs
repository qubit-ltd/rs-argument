// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Shared implementation of duration comparison validation.

use std::time::Duration;

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::ArgumentValue;
use crate::argument::ComparisonConstraint;

/// Applies one duration comparison and preserves the actual value on success.
///
/// `actual` and `bound` are passed to `predicate` exactly once. If the
/// predicate returns `false`, the error records `actual`, `constraint`, and
/// `path` without converting the duration to a unitless number.
///
/// # Type Parameters
///
/// - `F`: A comparison callable consumed exactly once with the actual duration
///   and its bound.
///
/// # Parameters
///
/// - `actual`: The duration being validated.
/// - `path`: The argument path to include in an error.
/// - `bound`: The duration compared with `actual`.
/// - `constraint`: The comparison constraint to store if validation fails.
/// - `predicate`: The comparison to apply to `actual` and `bound`.
///
/// # Returns
///
/// Returns `actual` unchanged when `predicate` returns `true`.
///
/// # Errors
///
/// Returns a comparison error containing `actual`, `constraint`, and `path`
/// when `predicate` returns `false`.
#[inline]
pub(in crate::argument) fn validate_duration_comparison<F>(
    actual: Duration,
    path: &str,
    bound: Duration,
    constraint: ComparisonConstraint,
    predicate: F,
) -> ArgumentResult<Duration>
where
    F: FnOnce(Duration, Duration) -> bool,
{
    if predicate(actual, bound) {
        Ok(actual)
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Comparison {
                actual: ArgumentValue::from(actual),
                constraint,
            },
        ))
    }
}
