// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Measurement units used by length validation.

/// Identifies how an observed string or collection length was measured.
///
/// The metric is retained in structured errors so equal numeric lengths and
/// constraints remain distinguishable across strings and collections.
/// This enum is non-exhaustive; downstream matches must include a wildcard arm.
///
/// ```compile_fail
/// use qubit_argument::LengthMetric;
///
/// fn label(metric: LengthMetric) -> &'static str {
///     match metric {
///         LengthMetric::Bytes => "bytes",
///         LengthMetric::UnicodeScalars => "Unicode scalars",
///         LengthMetric::Elements => "elements",
///     }
/// }
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthMetric {
    /// The number of bytes in a UTF-8 string.
    Bytes,
    /// The number of Unicode scalar values in a string.
    UnicodeScalars,
    /// The number of elements in a collection.
    Elements,
}
