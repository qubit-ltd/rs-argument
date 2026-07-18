// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for pattern expectation values.

use qubit_argument::PatternExpectation;

/// Verifies that match and non-match expectations remain distinct values.
#[test]
fn test_pattern_expectation_distinguishes_outcomes() {
    assert_ne!(PatternExpectation::Match, PatternExpectation::NoMatch);
}

/// Verifies that pattern expectations are copyable semantic values.
#[test]
fn test_pattern_expectation_is_copyable() {
    let expectation = PatternExpectation::NoMatch;
    let copied = expectation;

    assert_eq!(expectation, copied);
}
