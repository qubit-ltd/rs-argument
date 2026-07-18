// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for index role values.

use qubit_argument::IndexRole;

/// Verifies that index roles are copyable semantic values.
#[test]
fn test_index_role_is_copyable() {
    let role = IndexRole::Element;
    let copied = role;

    assert_eq!(role, copied);
}

/// Verifies that element and position indexes are represented distinctly.
#[test]
fn test_index_role_distinguishes_element_and_position() {
    assert_ne!(IndexRole::Element, IndexRole::Position);
}
