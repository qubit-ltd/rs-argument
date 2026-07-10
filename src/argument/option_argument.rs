// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for optional arguments.

use crate::argument::{ArgumentError, ArgumentErrorKind, ArgumentResult};

/// Validates optional arguments without requiring their values to be cloned.
///
/// A required value can be extracted with [`Self::require_some`]. Conditional
/// validation borrows a present value only for the validator call and returns
/// the original option on success.
pub trait OptionArgument<T>: Sized {
    /// Requires this option to contain a value.
    ///
    /// A present value is moved out and returned. An absent value returns
    /// [`ArgumentErrorKind::Missing`] at `path`.
    fn require_some(self, path: &str) -> ArgumentResult<T>;

    /// Validates a present value by temporary borrow.
    ///
    /// `validator` receives a shared reference when this option is present. A
    /// successful validator returns the original option without cloning its
    /// value; a validator error is propagated unchanged. An absent option is
    /// returned without executing `validator`.
    fn validate_if_some<F>(self, validator: F) -> ArgumentResult<Self>
    where
        F: FnOnce(&T) -> ArgumentResult<()>;
}

impl<T> OptionArgument<T> for Option<T> {
    /// Extracts a present value or reports that `path` is missing.
    ///
    /// The contained value is moved without cloning. An absent option returns
    /// [`ArgumentErrorKind::Missing`].
    #[inline]
    fn require_some(self, path: &str) -> ArgumentResult<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(ArgumentError::structured(path, ArgumentErrorKind::Missing)),
        }
    }

    /// Borrows and validates a present value, then returns the original option.
    ///
    /// A validator error is returned unchanged. When this option is absent,
    /// `validator` is not executed.
    #[inline]
    fn validate_if_some<F>(self, validator: F) -> ArgumentResult<Self>
    where
        F: FnOnce(&T) -> ArgumentResult<()>,
    {
        if let Some(value) = self.as_ref() {
            validator(value)?;
        }
        Ok(self)
    }
}

/// Validates a legacy optional value with a predicate when it is present.
///
/// `path` identifies predicate failures, `value` is returned unchanged on
/// success, `predicate` receives a shared reference only when `value` is
/// present, and `error_message` describes a rejected value. An absent value
/// skips `predicate`. A rejected value returns a structured custom error.
/// This transitional export is retained only until the public API cleanup.
#[doc(hidden)]
#[inline]
pub fn require_null_or<T, F>(
    path: &str,
    value: Option<T>,
    predicate: F,
    error_message: &str,
) -> ArgumentResult<Option<T>>
where
    F: FnOnce(&T) -> bool,
{
    if let Some(item) = value.as_ref()
        && !predicate(item)
    {
        return Err(ArgumentError::structured(
            path,
            ArgumentErrorKind::Custom {
                code: String::from("legacy_predicate"),
                message: String::from(error_message),
            },
        ));
    }
    Ok(value)
}
