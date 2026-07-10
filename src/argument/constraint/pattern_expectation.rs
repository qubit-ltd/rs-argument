// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Pattern expectations used by string validation.

/// Whether a string is expected to match a pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternExpectation {
    /// Requires the string to match the pattern.
    Match,
    /// Requires the string not to match the pattern.
    NoMatch,
}
