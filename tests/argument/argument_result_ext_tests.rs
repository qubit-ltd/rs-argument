// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for nested path propagation on argument results.

use qubit_argument::ArgumentError;
use qubit_argument::ArgumentErrorKind;
use qubit_argument::ArgumentResult;
use qubit_argument::ArgumentResultExt;

/// Verifies that successful results remain unchanged by path prefixing.
#[test]
fn test_with_path_prefix_preserves_success() {
    let result: ArgumentResult<usize> = Ok(4);
    assert_eq!(result.with_path_prefix("timeouts"), Ok(4));
}

/// Verifies that an error receives a nested prefix without changing its kind.
#[test]
fn test_with_path_prefix_prefixes_error() {
    let result: ArgumentResult<usize> = Err(ArgumentError::new("connect", ArgumentErrorKind::Missing));
    let error = result
        .with_path_prefix("timeouts")
        .expect_err("a missing nested value must remain an error");
    assert_eq!(error.path().as_str(), "timeouts.connect");
    assert_eq!(error.kind(), &ArgumentErrorKind::Missing);
}

/// Verifies direct error prefixing preserves the structured failure kind.
#[test]
fn test_with_path_prefix_preserves_error_kind() {
    let error = ArgumentError::new("host", ArgumentErrorKind::Blank).with_path_prefix("proxy");
    assert_eq!(error.path().as_str(), "proxy.host");
    assert_eq!(error.kind(), &ArgumentErrorKind::Blank);
}
