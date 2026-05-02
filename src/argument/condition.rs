/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Condition Argument Validation
//!
//! Provides general condition validation functionality.
//!
//! # Author
//!
//! Haixing Hu

use super::argument_error::{
    ArgumentError,
    ArgumentResult,
};

/// Check if an argument condition is true
///
/// This is the most basic validation function for checking arbitrary boolean conditions.
///
/// # Parameters
///
/// * `condition` - The condition to check
///
/// # Returns
///
/// Returns `Ok(())` if the condition is true, otherwise returns an error
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_argument;
///
/// let age = 25;
/// assert!(check_argument(age >= 18).is_ok());
/// assert!(check_argument(age < 18).is_err());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_argument(condition: bool) -> ArgumentResult<()> {
    if !condition {
        return Err(ArgumentError::new("Argument condition not satisfied"));
    }
    Ok(())
}

/// Check if an argument condition is true and provide a custom error message
///
/// # Parameters
///
/// * `condition` - The condition to check
/// * `message` - Error message when condition is not satisfied
///
/// # Returns
///
/// Returns `Ok(())` if the condition is true, otherwise returns an error with custom message
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_argument_with_message;
///
/// let count = 5;
/// let result = check_argument_with_message(
///     count > 0,
///     "Count must be positive"
/// );
/// assert!(result.is_ok());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_argument_with_message(condition: bool, message: &str) -> ArgumentResult<()> {
    if !condition {
        return Err(ArgumentError::new(message));
    }
    Ok(())
}

/// Check if an argument condition is true and use formatted message
///
/// # Parameters
///
/// * `condition` - The condition to check
/// * `format` - Format string
/// * `args` - Format arguments
///
/// # Returns
///
/// Returns `Ok(())` if the condition is true, otherwise returns formatted error message
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_argument_fmt;
///
/// let value = 150;
/// let max = 100;
/// let result = check_argument_fmt(
///     value <= max,
///     format!("Value {} exceeds maximum {}", value, max)
/// );
/// assert!(result.is_err());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_argument_fmt(condition: bool, message: String) -> ArgumentResult<()> {
    if !condition {
        return Err(ArgumentError::new(message));
    }
    Ok(())
}

/// Check if a state condition is true
///
/// Used to validate the state of an object or system. Similar to `check_argument` but semantically for state checking.
///
/// # Parameters
///
/// * `condition` - The state condition to check
///
/// # Returns
///
/// Returns `Ok(())` if the condition is true, otherwise returns an error
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_state;
///
/// let is_initialized = true;
/// assert!(check_state(is_initialized).is_ok());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_state(condition: bool) -> ArgumentResult<()> {
    if !condition {
        return Err(ArgumentError::new("State condition not satisfied"));
    }
    Ok(())
}

/// Check if a state condition is true and provide a custom error message
///
/// # Parameters
///
/// * `condition` - The state condition to check
/// * `message` - Error message when condition is not satisfied
///
/// # Returns
///
/// Returns `Ok(())` if the condition is true, otherwise returns an error with custom message
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_state_with_message;
///
/// let is_connected = false;
/// let result = check_state_with_message(
///     is_connected,
///     "Connection must be established first"
/// );
/// assert!(result.is_err());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_state_with_message(condition: bool, message: &str) -> ArgumentResult<()> {
    if !condition {
        return Err(ArgumentError::new(message));
    }
    Ok(())
}

/// Check boundary conditions
///
/// Validates that offset and length are within valid range to prevent array bounds errors.
///
/// # Parameters
///
/// * `offset` - Starting offset
/// * `length` - Length to access
/// * `total_length` - Total length
///
/// # Returns
///
/// Returns `Ok(())` if boundaries are valid, otherwise returns an error
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_bounds;
///
/// let buffer_len = 100;
/// assert!(check_bounds(10, 20, buffer_len).is_ok());
/// assert!(check_bounds(90, 20, buffer_len).is_err());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_bounds(offset: usize, length: usize, total_length: usize) -> ArgumentResult<()> {
    if offset > total_length {
        return Err(ArgumentError::new(format!(
            "Offset {} exceeds total length {}",
            offset, total_length
        )));
    }

    if length > total_length - offset {
        return Err(ArgumentError::new(format!(
            "Length {} starting from offset {} exceeds total length {}",
            length, offset, total_length
        )));
    }

    Ok(())
}

/// Check if an index is within valid range
///
/// # Parameters
///
/// * `index` - The index to check
/// * `size` - Collection size
///
/// # Returns
///
/// Returns the index itself if valid, otherwise returns an error
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_element_index;
///
/// let list_size = 10;
/// assert_eq!(check_element_index(5, list_size).unwrap(), 5);
/// assert!(check_element_index(10, list_size).is_err());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_element_index(index: usize, size: usize) -> ArgumentResult<usize> {
    if index >= size {
        return Err(ArgumentError::new(format!(
            "Index {} out of range [0, {})",
            index, size
        )));
    }
    Ok(index)
}

/// Check if a position index is within valid range
///
/// Position index can equal size (for insertion operations).
///
/// # Parameters
///
/// * `index` - The position index to check
/// * `size` - Collection size
///
/// # Returns
///
/// Returns the index itself if valid, otherwise returns an error
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_position_index;
///
/// let list_size = 10;
/// assert_eq!(check_position_index(10, list_size).unwrap(), 10); // Can equal size
/// assert!(check_position_index(11, list_size).is_err());
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_position_index(index: usize, size: usize) -> ArgumentResult<usize> {
    if index > size {
        return Err(ArgumentError::new(format!(
            "Position index {} out of range [0, {}]",
            index, size
        )));
    }
    Ok(index)
}

/// Check if a position index range is valid
///
/// # Parameters
///
/// * `start` - Start index
/// * `end` - End index
/// * `size` - Collection size
///
/// # Returns
///
/// Returns `Ok(())` if the range is valid, otherwise returns an error
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::check_position_indexes;
///
/// let list_size = 10;
/// assert!(check_position_indexes(2, 5, list_size).is_ok());
/// assert!(check_position_indexes(5, 2, list_size).is_err()); // start > end
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[inline]
pub fn check_position_indexes(start: usize, end: usize, size: usize) -> ArgumentResult<()> {
    if start > end {
        return Err(ArgumentError::new(format!(
            "Start index {} is greater than end index {}",
            start, end
        )));
    }

    if end > size {
        return Err(ArgumentError::new(format!(
            "End index {} out of range [0, {}]",
            end, size
        )));
    }

    Ok(())
}
