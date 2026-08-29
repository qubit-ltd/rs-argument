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

/// Validates collection lengths while preserving the original collection.
///
/// Every successful method consumes and returns the same collection value
/// without cloning its elements. Implementations are provided for owned
/// vectors, borrowed slices, and arrays. Length failures carry
/// [`LengthMetric::Elements`].
///
/// The trait is sealed to those library-supported collection forms.
pub trait CollectionArgument: Sealed + Sized {
    /// Requires this collection to contain at least one element.
    ///
    /// Success returns the original collection without cloning its elements.
    /// An empty collection returns [`ArgumentErrorKind::Empty`] at `path`.
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self>;

    /// Requires this collection to contain exactly `expected` elements.
    ///
    /// Success returns the original collection without cloning its elements.
    /// A different length returns [`ArgumentErrorKind::Length`] at `path` with
    /// an exact constraint.
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self>;

    /// Requires this collection to contain at least `min` elements.
    ///
    /// Success returns the original collection without cloning its elements.
    /// A shorter collection returns [`ArgumentErrorKind::Length`] at `path`
    /// with a minimum constraint.
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self>;

    /// Requires this collection to contain at most `max` elements.
    ///
    /// Success returns the original collection without cloning its elements.
    /// A longer collection returns [`ArgumentErrorKind::Length`] at `path`
    /// with a maximum constraint.
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self>;

    /// Requires this collection's length to lie in `min..=max`.
    ///
    /// The range is validated before the observed length. If `min > max`, this
    /// returns [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`;
    /// otherwise, an out-of-range length returns
    /// [`ArgumentErrorKind::Length`]. Success returns the original collection
    /// without cloning its elements.
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self>;
}

impl<T> CollectionArgument for Vec<T> {
    /// Validates non-emptiness and returns the original vector.
    ///
    /// An empty vector returns [`ArgumentErrorKind::Empty`] at `path`.
    #[inline]
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self> {
        if self.is_empty() {
            return Err(ArgumentError::new(path, ArgumentErrorKind::Empty));
        }
        Ok(self)
    }

    /// Validates the exact length and returns the original vector.
    ///
    /// A mismatch returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::Exact(expected))?;
        Ok(self)
    }

    /// Validates the minimum length and returns the original vector.
    ///
    /// A length below `min` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtLeast(min))?;
        Ok(self)
    }

    /// Validates the maximum length and returns the original vector.
    ///
    /// A length above `max` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtMost(max))?;
        Ok(self)
    }

    /// Validates an inclusive length range and returns the original vector.
    ///
    /// A reversed range returns
    /// [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`; an
    /// out-of-range length returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::InRange { min, max })?;
        Ok(self)
    }
}

impl<T> CollectionArgument for &[T] {
    /// Validates non-emptiness and returns the original borrowed slice.
    ///
    /// An empty slice returns [`ArgumentErrorKind::Empty`] at `path`.
    #[inline]
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self> {
        if self.is_empty() {
            return Err(ArgumentError::new(path, ArgumentErrorKind::Empty));
        }
        Ok(self)
    }

    /// Validates the exact length and returns the original borrowed slice.
    ///
    /// A mismatch returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::Exact(expected))?;
        Ok(self)
    }

    /// Validates the minimum length and returns the original borrowed slice.
    ///
    /// A length below `min` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtLeast(min))?;
        Ok(self)
    }

    /// Validates the maximum length and returns the original borrowed slice.
    ///
    /// A length above `max` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::AtMost(max))?;
        Ok(self)
    }

    /// Validates an inclusive length range and returns the borrowed slice.
    ///
    /// A reversed range returns
    /// [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`; an
    /// out-of-range length returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self> {
        validate_length(path, self.len(), LengthConstraint::InRange { min, max })?;
        Ok(self)
    }
}

impl<T, const N: usize> CollectionArgument for [T; N] {
    /// Validates non-emptiness and returns the original array.
    ///
    /// A zero-length array returns [`ArgumentErrorKind::Empty`] at `path`.
    #[inline]
    fn require_non_empty(self, path: &str) -> ArgumentResult<Self> {
        if N == 0 {
            return Err(ArgumentError::new(path, ArgumentErrorKind::Empty));
        }
        Ok(self)
    }

    /// Validates the exact length and returns the original array.
    ///
    /// A mismatch returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len(self, path: &str, expected: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::Exact(expected))?;
        Ok(self)
    }

    /// Validates the minimum length and returns the original array.
    ///
    /// A length below `min` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len_at_least(self, path: &str, min: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::AtLeast(min))?;
        Ok(self)
    }

    /// Validates the maximum length and returns the original array.
    ///
    /// A length above `max` returns [`ArgumentErrorKind::Length`] at `path`.
    #[inline]
    fn require_len_at_most(self, path: &str, max: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::AtMost(max))?;
        Ok(self)
    }

    /// Validates an inclusive length range and returns the original array.
    ///
    /// A reversed range returns
    /// [`ArgumentErrorKind::InvalidLengthConstraint`] at `path`; an
    /// out-of-range length returns [`ArgumentErrorKind::Length`].
    #[inline]
    fn require_len_in(self, path: &str, min: usize, max: usize) -> ArgumentResult<Self> {
        validate_length(path, N, LengthConstraint::InRange { min, max })?;
        Ok(self)
    }
}

/// Validates an observed collection length against a structured constraint.
///
/// `path` identifies any failure, `actual` is the observed element count, and
/// `constraint` describes the required relationship. A reversed inclusive
/// range returns [`ArgumentErrorKind::InvalidLengthConstraint`] before
/// `actual` is checked. Any other unsatisfied constraint returns
/// [`ArgumentErrorKind::Length`]; a satisfied constraint returns `Ok(())`.
fn validate_length(path: &str, actual: usize, constraint: LengthConstraint) -> ArgumentResult<()> {
    if let LengthConstraint::InRange { min, max } = &constraint
        && min > max
    {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::InvalidLengthConstraint {
                constraint,
                metric: LengthMetric::Elements,
            },
        ));
    }

    let is_valid = match &constraint {
        LengthConstraint::Exact(expected) => actual == *expected,
        LengthConstraint::AtLeast(min) => actual >= *min,
        LengthConstraint::AtMost(max) => actual <= *max,
        LengthConstraint::InRange { min, max } => actual >= *min && actual <= *max,
    };
    if is_valid {
        Ok(())
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Length {
                actual,
                constraint,
                metric: LengthMetric::Elements,
            },
        ))
    }
}
