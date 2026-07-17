// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Structured paths that identify arguments in validation errors.

use std::fmt::{
    self,
    Display,
    Formatter,
};

/// An owned path identifying an argument or one of its nested components.
///
/// The path is stored exactly as supplied; this type does not parse or
/// normalize separators.
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_argument::ArgumentPath;
///
/// ArgumentPath::new("value");
/// ```
#[must_use]
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
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Prepends a parent path component to this path.
    ///
    /// # Parameters
    ///
    /// - `prefix`: The parent path to place before the stored path.
    ///
    /// # Returns
    ///
    /// The composed path. Non-empty components are separated by one dot. If
    /// either component is empty, no separator is added.
    #[inline]
    pub fn with_prefix(self, prefix: &str) -> Self {
        if prefix.is_empty() {
            return self;
        }
        if self.0.is_empty() {
            return Self(prefix.to_owned());
        }
        let mut path = String::with_capacity(prefix.len() + 1 + self.0.len());
        path.push_str(prefix);
        path.push('.');
        path.push_str(&self.0);
        Self(path)
    }
}

impl AsRef<str> for ArgumentPath {
    /// Borrows the stored path as a string slice.
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ArgumentPath {
    /// Writes the stored path text without additional decoration.
    #[inline(always)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
