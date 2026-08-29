// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Simulated downstream error-conversion tests.

use qubit_argument::ArgumentError;
use qubit_argument::ArgumentErrorKind;
use qubit_argument::ArgumentValue;
use qubit_argument::ComparisonConstraint;

#[derive(Debug, PartialEq, Eq)]
enum WrappedDomainError {
    InvalidArgument(ArgumentError),
}

impl From<ArgumentError> for WrappedDomainError {
    /// Wraps an argument error without discarding its structured fields.
    fn from(error: ArgumentError) -> Self {
        Self::InvalidArgument(error)
    }
}

/// Produces a missing-argument error and converts it with the `?` operator.
fn fail_with_missing_argument() -> Result<(), WrappedDomainError> {
    Err(ArgumentError::new("token", ArgumentErrorKind::Missing))?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum DomainError {
    ZeroPoolSize,
    InvalidArgument(ArgumentError),
}

impl From<ArgumentError> for DomainError {
    /// Maps the pool-size comparison failure and preserves all other errors.
    fn from(error: ArgumentError) -> Self {
        if error.path().as_str() == "pool_size" && matches!(error.kind(), ArgumentErrorKind::Comparison { .. }) {
            Self::ZeroPoolSize
        } else {
            Self::InvalidArgument(error)
        }
    }
}

/// Produces a zero-pool-size comparison error converted by `?`.
fn fail_with_zero_pool_size() -> Result<(), DomainError> {
    Err(ArgumentError::new(
        "pool_size",
        ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(0_usize),
            constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(0_usize)),
        },
    ))?;
    Ok(())
}

/// Produces an unrelated error to exercise the conversion fallback.
fn fail_with_other_argument() -> Result<(), DomainError> {
    Err(ArgumentError::new("token", ArgumentErrorKind::Missing))?;
    Ok(())
}

/// Verifies transparent downstream wrapping through the `?` operator.
#[test]
fn test_from_wraps_argument_error_with_question_mark() {
    let error = fail_with_missing_argument().expect_err("missing token must fail");
    assert_eq!(
        error,
        WrappedDomainError::InvalidArgument(ArgumentError::new("token", ArgumentErrorKind::Missing,)),
    );
}

/// Verifies a downstream conversion can specialize by path and error kind.
#[test]
fn test_from_maps_structured_error_with_question_mark() {
    let error = fail_with_zero_pool_size().expect_err("zero pool size must fail");
    assert_eq!(error, DomainError::ZeroPoolSize);
}

/// Verifies unmatched structured errors retain their original information.
#[test]
fn test_from_preserves_unmatched_argument_error() {
    let error = fail_with_other_argument().expect_err("missing token must fail");
    assert_eq!(
        error,
        DomainError::InvalidArgument(ArgumentError::new("token", ArgumentErrorKind::Missing)),
    );
}
