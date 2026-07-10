// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured argument validation errors.

use qubit_argument::{ArgumentError, ArgumentErrorKind, ArgumentPath, LengthConstraint};

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
