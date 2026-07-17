// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Numeric range bound constraints.

use crate::argument::ArgumentValue;

/// One side of a numeric range constraint.
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_argument::ArgumentBound;
///
/// ArgumentBound::Unbounded;
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentBound {
    /// Places no limit on this side of the range.
    Unbounded,
    /// Includes the captured endpoint in the range.
    Included(ArgumentValue),
    /// Excludes the captured endpoint from the range.
    Excluded(ArgumentValue),
}
