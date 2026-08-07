// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for optional arguments.

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::internal::Sealed;

/// Validates optional arguments without requiring their values to be cloned.
///
/// A required value can be extracted with [`Self::require_some`]. Conditional
/// validation borrows a present value only for the validator call and returns
/// the original option on success.
///
/// The trait is sealed and implemented only for `Option<T>`.
pub trait OptionArgument<T>: Sealed + Sized {
    /// Requires this option to contain a value.
    ///
    /// A present value is moved out and returned without cloning. An absent
    /// value returns [`ArgumentErrorKind::Missing`] at `path`.
    fn require_some(self, path: &str) -> ArgumentResult<T>;

    /// Validates a present value by temporary borrow.
    ///
    /// `validator` receives a shared reference when this option is present. A
    /// successful validator returns the original option without cloning its
    /// value; it introduces no failure kind of its own and propagates any
    /// [`ArgumentErrorKind`] returned by `validator` unchanged. An absent
    /// option is returned without executing `validator`.
    fn validate_if_some<F>(self, validator: F) -> ArgumentResult<Self>
    where
        F: FnOnce(&T) -> ArgumentResult<()>;

    /// Validates and returns a present owned value.
    ///
    /// `validator` receives ownership of the contained value when this option
    /// is present and must return the validated value. A successful validator
    /// is wrapped in [`Some`] and returned; it may transform the value without
    /// requiring [`Clone`] or [`Copy`]. An absent option is returned without
    /// executing `validator`. Validator errors are propagated unchanged.
    fn validate_some<F>(self, validator: F) -> ArgumentResult<Self>
    where
        F: FnOnce(T) -> ArgumentResult<T>;
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
            None => Err(ArgumentError::new(path, ArgumentErrorKind::Missing)),
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

    /// Moves a present value through `validator` and rewraps it on success.
    ///
    /// A validator error is returned unchanged. When this option is absent,
    /// `validator` is not executed.
    #[inline]
    fn validate_some<F>(self, validator: F) -> ArgumentResult<Self>
    where
        F: FnOnce(T) -> ArgumentResult<T>,
    {
        match self {
            Some(value) => validator(value).map(Some),
            None => Ok(None),
        }
    }
}
