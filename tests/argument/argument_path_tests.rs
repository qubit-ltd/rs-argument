// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for structured argument paths.

use qubit_argument::ArgumentPath;

/// Verifies that an argument path owns and exposes its path text.
#[test]
fn test_argument_path_exposes_owned_path() {
    let path = ArgumentPath::new("retry.max_attempts");
    assert_eq!(path.as_str(), "retry.max_attempts");
    assert_eq!(path.as_ref(), "retry.max_attempts");
    assert_eq!(path.to_string(), "retry.max_attempts");
}
