// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for collection arguments.

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::LengthConstraint;
use crate::argument::LengthMetric;
use crate::argument::internal::Sealed;
use crate::argument::internal::validate_length;

/// Validates collection lengths while preserving the original collection.
///
/// Every successful method consumes and returns the same collection value
/// without cloning its elements. Implementations are provided for owned
/// vectors, borrowed slices, and arrays. Length failures carry
/// [`LengthMetric::Elements`].
///
/// Validation failures return only an error, not the original collection.
/// Owned vectors and arrays, including their elements, are dropped on failure.
/// For borrowed slices, the caller retains ownership of the underlying
/// collection and its elements on both success and failure.
///
/// The trait is sealed to those library-supported collection forms.
///
/// # Examples
///
/// ```
/// use qubit_argument::CollectionArgument;
///
/// let values = vec![1, 2, 3].require_len("items", 3).unwrap();
/// assert_eq!(values, [1, 2, 3]);
/// ```
pub trait CollectionArgument: Sealed + Sized {
    /// Requires this collection to contain at least one element.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Empty`] at `path` when the collection is
    /// empty.
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this collection to contain exactly `expected` elements.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `expected`: the required number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length differs
    /// from `expected`.
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self>;

    /// Requires this collection to contain at least `min` elements.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the minimum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// below `min`.
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self>;

    /// Requires this collection to contain at most `max` elements.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `max`: the maximum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// above `max`.
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self>;

    /// Requires this collection's length to lie in `min..=max`.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the inclusive lower bound.
    /// - `max`: the inclusive upper bound.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// The range is validated before the observed length. If `min > max`,
    /// returns [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`;
    /// otherwise, an out-of-range length returns [`ArgumentErrorKind::Length`].
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self>;
}

impl<T> CollectionArgument for Vec<T> {
    /// Validates non-emptiness and returns the original vector.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Empty`] at `path` when the collection is
    /// empty.
    #[inline]
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self> {
        if self.is_empty() {
            return Err(ArgumentError::new(path, ArgumentErrorKind::Empty));
        }
        Ok(self)
    }

    /// Validates the exact length and returns the original vector.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `expected`: the required number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length differs
    /// from `expected`.
    #[inline]
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::Exact(expected),
            LengthMetric::Elements,
        )?;
        Ok(self)
    }

    /// Validates the minimum length and returns the original vector.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the minimum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// below `min`.
    #[inline]
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtLeast(min), LengthMetric::Elements)?;
        Ok(self)
    }

    /// Validates the maximum length and returns the original vector.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `max`: the maximum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// above `max`.
    #[inline]
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtMost(max), LengthMetric::Elements)?;
        Ok(self)
    }

    /// Validates an inclusive length range and returns the original vector.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the inclusive lower bound.
    /// - `max`: the inclusive upper bound.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// The range is validated before the observed length. If `min > max`,
    /// returns [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`;
    /// otherwise, an out-of-range length returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::InRange { min, max },
            LengthMetric::Elements,
        )?;
        Ok(self)
    }
}

impl<T> CollectionArgument for &[T] {
    /// Validates non-emptiness and returns the original borrowed slice.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Empty`] at `path` when the collection is
    /// empty.
    #[inline]
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self> {
        if self.is_empty() {
            return Err(ArgumentError::new(path, ArgumentErrorKind::Empty));
        }
        Ok(self)
    }

    /// Validates the exact length and returns the original borrowed slice.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `expected`: the required number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length differs
    /// from `expected`.
    #[inline]
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::Exact(expected),
            LengthMetric::Elements,
        )?;
        Ok(self)
    }

    /// Validates the minimum length and returns the original borrowed slice.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the minimum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// below `min`.
    #[inline]
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtLeast(min), LengthMetric::Elements)?;
        Ok(self)
    }

    /// Validates the maximum length and returns the original borrowed slice.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `max`: the maximum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// above `max`.
    #[inline]
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtMost(max), LengthMetric::Elements)?;
        Ok(self)
    }

    /// Validates an inclusive length range and returns the borrowed slice.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the inclusive lower bound.
    /// - `max`: the inclusive upper bound.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// The range is validated before the observed length. If `min > max`,
    /// returns [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`;
    /// otherwise, an out-of-range length returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self> {
        validate_length(
            path,
            self.len(),
            LengthConstraint::InRange { min, max },
            LengthMetric::Elements,
        )?;
        Ok(self)
    }
}

impl<T, const N: usize> CollectionArgument for [T; N] {
    /// Validates non-emptiness and returns the original array.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Empty`] at `path` when the collection is
    /// empty.
    #[inline]
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self> {
        if N == 0 {
            return Err(ArgumentError::new(path, ArgumentErrorKind::Empty));
        }
        Ok(self)
    }

    /// Validates the exact length and returns the original array.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `expected`: the required number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length differs
    /// from `expected`.
    #[inline]
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::Exact(expected), LengthMetric::Elements)?;
        Ok(self)
    }

    /// Validates the minimum length and returns the original array.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the minimum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// below `min`.
    #[inline]
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::AtLeast(min), LengthMetric::Elements)?;
        Ok(self)
    }

    /// Validates the maximum length and returns the original array.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `max`: the maximum number of elements.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentErrorKind::Length`] at `path` when the length is
    /// above `max`.
    #[inline]
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::AtMost(max), LengthMetric::Elements)?;
        Ok(self)
    }

    /// Validates an inclusive length range and returns the original array.
    ///
    /// # Parameters
    ///
    /// - `path`: identifies the collection in a validation error.
    /// - `min`: the inclusive lower bound.
    /// - `max`: the inclusive upper bound.
    ///
    /// # Returns
    ///
    /// The original collection on success, without cloning its elements.
    ///
    /// # Errors
    ///
    /// The range is validated before the observed length. If `min > max`,
    /// returns [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`;
    /// otherwise, an out-of-range length returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::InRange { min, max }, LengthMetric::Elements)?;
        Ok(self)
    }
}
