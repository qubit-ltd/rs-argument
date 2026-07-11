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

/// Verifies that a non-empty prefix is joined with one path separator.
#[test]
fn test_with_prefix_joins_nested_path() {
    let path = ArgumentPath::new("max_attempts").with_prefix("retry");
    assert_eq!(path.as_str(), "retry.max_attempts");
}

/// Verifies that empty path components do not create stray separators.
#[test]
fn test_with_prefix_handles_empty_components() {
    let unchanged = ArgumentPath::new("timeout").with_prefix("");
    assert_eq!(unchanged.as_str(), "timeout");

    let prefix_only = ArgumentPath::new("").with_prefix("request");
    assert_eq!(prefix_only.as_str(), "request");

    let empty = ArgumentPath::new("").with_prefix("");
    assert_eq!(empty.as_str(), "");
}
