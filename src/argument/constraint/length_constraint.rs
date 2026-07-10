// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String and collection length constraints.

/// A constraint on the length of a string or collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthConstraint {
    /// Requires exactly the specified length.
    Exact(usize),
    /// Requires at least the specified length.
    AtLeast(usize),
    /// Requires at most the specified length.
    AtMost(usize),
    /// Requires a length between `min` and `max`, inclusive.
    InRange {
        /// The inclusive minimum length.
        min: usize,
        /// The inclusive maximum length.
        max: usize,
    },
}
