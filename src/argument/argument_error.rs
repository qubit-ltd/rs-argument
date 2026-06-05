// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Argument Validation Errors
//!
//! Defines error types related to argument validation.

use std::fmt;

/// Argument validation error
///
/// Returned when an argument does not satisfy validation conditions.
///
/// # Features
///
/// - Contains detailed error messages
/// - Implements standard error traits
/// - Supports conversion from strings
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::ArgumentError;
///
/// let error = ArgumentError::new("Invalid argument");
/// assert_eq!(error.message(), "Invalid argument");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentError {
    message: String,
}

impl ArgumentError {
    /// Create a new argument error
    ///
    /// # Parameters
    ///
    /// * `message` - Error message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::ArgumentError;
    ///
    /// let error = ArgumentError::new("Value cannot be negative");
    /// ```
    #[inline]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Get the error message
    ///
    /// # Returns
    ///
    /// Returns a reference to the error message
    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ArgumentError {}

impl From<String> for ArgumentError {
    #[inline]
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<&str> for ArgumentError {
    #[inline]
    fn from(message: &str) -> Self {
        Self::new(message)
    }
}

/// Argument validation result type
///
/// Unified result type for all argument validation operations.
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::{ArgumentResult, ArgumentError};
///
/// fn validate_positive(value: i32) -> ArgumentResult<i32> {
///     if value > 0 {
///         Ok(value)
///     } else {
///         Err(ArgumentError::new("Value must be positive"))
///     }
/// }
/// ```
pub type ArgumentResult<T> = Result<T, ArgumentError>;
