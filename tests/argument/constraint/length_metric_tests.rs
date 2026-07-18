// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for length metric values.

use qubit_argument::LengthMetric;

/// Verifies that every length metric is a distinct copyable value.
#[test]
fn test_length_metric_distinguishes_measurement_units() {
    let bytes = LengthMetric::Bytes;
    let unicode_scalars = LengthMetric::UnicodeScalars;
    let elements = LengthMetric::Elements;

    assert_eq!(bytes, LengthMetric::Bytes);
    assert_ne!(bytes, unicode_scalars);
    assert_ne!(bytes, elements);
    assert_ne!(unicode_scalars, elements);
}
