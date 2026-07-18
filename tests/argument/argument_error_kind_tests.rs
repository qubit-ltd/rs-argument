// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for argument error kind values.

use qubit_argument::{
    ArgumentErrorKind,
    LengthConstraint,
    LengthMetric,
};

/// Verifies that simple argument error kinds retain their derived semantics.
#[test]
fn test_argument_error_kind_simple_variants_are_distinct() {
    assert_ne!(ArgumentErrorKind::Missing, ArgumentErrorKind::Blank);
    assert_ne!(ArgumentErrorKind::Blank, ArgumentErrorKind::Empty);
}

/// Verifies that structured length error kinds preserve all captured fields.
#[test]
fn test_argument_error_kind_length_preserves_structured_fields() {
    let kind = ArgumentErrorKind::Length {
        actual: 3,
        constraint: LengthConstraint::AtMost(2),
        metric: LengthMetric::Elements,
    };

    assert!(matches!(
        kind,
        ArgumentErrorKind::Length {
            actual: 3,
            constraint: LengthConstraint::AtMost(2),
            metric: LengthMetric::Elements,
        },
    ));
}
