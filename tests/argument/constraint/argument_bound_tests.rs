// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for numeric range bound constraints.

use qubit_argument::ArgumentBound;
use qubit_argument::ArgumentValue;

/// Verifies that argument bounds distinguish unbounded, included, and excluded
/// endpoints.
#[test]
fn test_argument_bound_distinguishes_bound_kinds() {
    let value = ArgumentValue::from(5_i32);

    assert_ne!(ArgumentBound::Unbounded, ArgumentBound::Included(value));
    assert_ne!(ArgumentBound::Included(value), ArgumentBound::Excluded(value));
}

/// Verifies that included and excluded bounds retain their endpoint value.
#[test]
fn test_argument_bound_preserves_endpoint_value() {
    let included = ArgumentBound::Included(ArgumentValue::from(-1_i32));
    let excluded = ArgumentBound::Excluded(ArgumentValue::from(2_i32));

    assert!(matches!(included, ArgumentBound::Included(ArgumentValue::Signed(-1)),));
    assert!(matches!(excluded, ArgumentBound::Excluded(ArgumentValue::Signed(2)),));
}
