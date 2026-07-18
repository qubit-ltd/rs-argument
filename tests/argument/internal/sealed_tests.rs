// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for public validation traits backed by private sealing.

use qubit_argument::{
    CollectionArgument,
    NumericArgument,
    OptionArgument,
    StringArgument,
};

/// Verifies that sealed public validation traits are implemented for the
/// documented primitive and standard-library types.
#[test]
fn test_sealed_traits_support_documented_receiver_types() {
    assert_eq!(1_i32.require_positive("count").expect("one is positive"), 1);
    assert_eq!(
        "name"
            .require_non_blank("name")
            .expect("non-blank string is valid"),
        "name",
    );
    assert_eq!(
        vec![1, 2]
            .require_non_empty("items")
            .expect("non-empty vector is valid"),
        vec![1, 2],
    );
    assert_eq!(
        Some("value")
            .require_some("option")
            .expect("present option is valid"),
        "value",
    );
}
