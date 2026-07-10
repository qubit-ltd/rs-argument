// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured paths that identify arguments in validation errors.

use std::fmt::{self, Display, Formatter};

/// An owned path identifying an argument or one of its nested components.
///
/// The path is stored exactly as supplied; this type does not parse or
/// normalize separators.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgumentPath(String);

impl ArgumentPath {
    /// Creates an argument path by copying `path` without normalization.
    ///
    /// # Parameters
    ///
    /// - `path`: The path text to store.
    ///
    /// # Returns
    ///
    /// A new owned argument path.
    #[inline]
    pub fn new(path: &str) -> Self {
        Self(path.to_owned())
    }

    /// Returns the stored path text.
    ///
    /// The returned string slice remains valid for the lifetime of this path.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for ArgumentPath {
    /// Borrows the stored path as a string slice.
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ArgumentPath {
    /// Writes the stored path text without additional decoration.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
