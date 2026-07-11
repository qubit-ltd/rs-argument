// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Nested path propagation for argument validation results.

use crate::argument::{
    ArgumentResult,
    sealed::Sealed,
};

/// Adds nested argument-path context to validation results.
///
/// Successful results are returned unchanged and do not allocate a path.
/// Failed results consume their [`crate::ArgumentError`] and prepend the
/// supplied parent path while preserving the structured failure kind.
///
/// The trait is sealed and implemented only for [`ArgumentResult`].
pub trait ArgumentResultExt<T>: Sealed + Sized {
    /// Prepends a parent path to a validation failure.
    ///
    /// # Parameters
    ///
    /// - `prefix`: The parent path to add before an error's current path.
    ///
    /// # Returns
    ///
    /// The unchanged successful value, or the original validation error with
    /// its path prefixed.
    fn with_path_prefix(self, prefix: &str) -> ArgumentResult<T>;
}

impl<T> ArgumentResultExt<T> for ArgumentResult<T> {
    /// Prefixes only the error branch and leaves success values untouched.
    #[inline]
    fn with_path_prefix(self, prefix: &str) -> ArgumentResult<T> {
        self.map_err(|error| error.with_path_prefix(prefix))
    }
}
