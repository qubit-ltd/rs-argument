// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Bounds and caller-defined argument validation.

use std::ops::Range;

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::IndexRole;

/// Validates a value with a caller-provided predicate.
///
/// The predicate receives a shared reference to `value`. If it returns `true`,
/// this function returns the original value without cloning it or allocating
/// error strings. If it returns `false`, `path`, `code`, and `message` identify
/// the failed validation rule.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::Custom`] when `predicate` returns `false`.
/// The error owns copies of `path`, `code`, and `message`.
#[inline]
pub fn require_that<T, F>(value: T, path: &str, predicate: F, code: &str, message: &str) -> ArgumentResult<T>
where
    F: FnOnce(&T) -> bool,
{
    if predicate(&value) {
        Ok(value)
    } else {
        Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Custom {
                code: String::from(code),
                message: String::from(message),
            },
        ))
    }
}

/// Validates that an offset and length fit within a total length.
///
/// `offset` may equal `total_length` only when `length` is zero. Validation
/// checks the offset before subtracting it from the total, so it never computes
/// an unchecked sum of the offset and length.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::Bounds`] when `offset` exceeds `total_length`
/// or when `length` exceeds the number of units remaining after `offset`.
#[inline]
pub fn check_bounds(path: &str, offset: usize, length: usize, total_length: usize) -> ArgumentResult<()> {
    if offset > total_length {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Bounds {
                offset,
                length,
                total_length,
            },
        ));
    }
    if length > total_length - offset {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Bounds {
                offset,
                length,
                total_length,
            },
        ));
    }
    Ok(())
}

/// Validates an index that must identify an existing element.
///
/// `index` is valid exactly when it is strictly less than `size`. On success,
/// the validated index is returned unchanged.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::Index`] with [`IndexRole::Element`] when
/// `index` is greater than or equal to `size`.
#[inline]
pub fn check_element_index(path: &str, index: usize, size: usize) -> ArgumentResult<usize> {
    if index >= size {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Index {
                index,
                size,
                role: IndexRole::Element,
            },
        ));
    }
    Ok(index)
}

/// Validates an index that identifies a boundary position.
///
/// `index` is valid when it is less than or equal to `size`, including the
/// position immediately after the final element. On success, the validated
/// index is returned unchanged.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::Index`] with [`IndexRole::Position`] when
/// `index` exceeds `size`.
#[inline]
pub fn check_position_index(path: &str, index: usize, size: usize) -> ArgumentResult<usize> {
    if index > size {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::Index {
                index,
                size,
                role: IndexRole::Position,
            },
        ));
    }
    Ok(index)
}

/// Validates a half-open range of boundary positions.
///
/// `start` and `end` are valid when `start <= end <= size`. Equal endpoints
/// are accepted and produce an empty range. On success, this function returns
/// the validated `start..end` range.
///
/// # Errors
///
/// Returns [`ArgumentErrorKind::IndexRange`] when `start` exceeds `end` or
/// when `end` exceeds `size`.
#[inline]
pub fn check_position_range(path: &str, start: usize, end: usize, size: usize) -> ArgumentResult<Range<usize>> {
    if start > end {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::IndexRange { start, end, size },
        ));
    }
    if end > size {
        return Err(ArgumentError::new(
            path,
            ArgumentErrorKind::IndexRange { start, end, size },
        ));
    }
    Ok(start..end)
}
