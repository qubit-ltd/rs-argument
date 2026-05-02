/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Option Argument Validation
//!
//! Provides validation functionality for Option type arguments.
//!

use super::argument_error::{
    ArgumentError,
    ArgumentResult,
};

/// Option type argument validation extension
///
/// Provides null checking and condition validation for `Option<T>`.
///
/// # Features
///
/// - Null checking
/// - Condition validation
/// - Method chaining support
/// - Type safety
///
/// # Use Cases
///
/// - Optional parameter validation
/// - Configuration item checking
/// - User input processing
///
///
/// # Examples
///
/// Basic usage (returns `ArgumentResult`):
///
/// ```rust
/// use qubit_argument::argument::{OptionArgument, ArgumentResult};
///
/// fn process_config(timeout: Option<u64>) -> ArgumentResult<()> {
///     let timeout = timeout.require_non_null("timeout")?;
///     println!("Timeout: {} seconds", timeout);
///     Ok(())
/// }
/// ```
///
/// Converting to other error types:
///
/// ```rust
/// use qubit_argument::argument::OptionArgument;
///
/// fn process_config(timeout: Option<u64>) -> Result<(), String> {
///     let timeout = timeout
///         .require_non_null("timeout")
///         .map_err(|e| e.to_string())?;
///     println!("Timeout: {} seconds", timeout);
///     Ok(())
/// }
/// ```
pub trait OptionArgument<T> {
    /// Validate that Option is not None
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns value if Some(value), otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::OptionArgument;
    ///
    /// let value: Option<i32> = Some(42);
    /// assert_eq!(value.require_non_null("value").unwrap(), 42);
    ///
    /// let none_value: Option<i32> = None;
    /// assert!(none_value.require_non_null("value").is_err());
    /// ```
    fn require_non_null(self, name: &str) -> ArgumentResult<T>;

    /// Validate that Option is not None and internal value satisfies condition
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `predicate` - Validation condition function
    /// * `error_msg` - Error message when condition is not satisfied
    ///
    /// # Returns
    ///
    /// Returns value if Some(value) and condition is satisfied, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::OptionArgument;
    ///
    /// let age: Option<u8> = Some(25);
    /// let result = age.require_non_null_and(
    ///     "age",
    ///     |&a| a >= 18,
    ///     "Must be at least 18 years old"
    /// );
    /// assert!(result.is_ok());
    /// ```
    fn require_non_null_and<F>(
        self,
        name: &str,
        predicate: F,
        error_msg: &str,
    ) -> ArgumentResult<T>
    where
        F: FnOnce(&T) -> bool;

    /// If Option is Some, validate the value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `validator` - Validation function that returns Result
    ///
    /// # Returns
    ///
    /// Returns None if None, returns validation result if Some
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::OptionArgument;
    ///
    /// let port: Option<u16> = Some(8080);
    /// let result = port.validate_if_present("port", |p| {
    ///     if *p < 1024 {
    ///         Err("Port must be greater than or equal to 1024".into())
    ///     } else {
    ///         Ok(*p)
    ///     }
    /// });
    /// assert!(result.is_ok());
    /// ```
    fn validate_if_present<F>(self, name: &str, validator: F) -> ArgumentResult<Option<T>>
    where
        F: FnOnce(&T) -> ArgumentResult<T>;
}

impl<T> OptionArgument<T> for Option<T> {
    #[inline]
    fn require_non_null(self, name: &str) -> ArgumentResult<T> {
        match self {
            Some(value) => Ok(value),
            None => {
                let message = format!("Parameter '{}' cannot be null", name);
                Err(ArgumentError::new(message))
            }
        }
    }

    #[inline]
    fn require_non_null_and<F>(self, name: &str, predicate: F, error_msg: &str) -> ArgumentResult<T>
    where
        F: FnOnce(&T) -> bool,
    {
        match self {
            Some(value) => {
                if predicate(&value) {
                    Ok(value)
                } else {
                    let message = format!("Parameter '{}' {}", name, error_msg);
                    Err(ArgumentError::new(message))
                }
            }
            None => {
                let message = format!("Parameter '{}' cannot be null", name);
                Err(ArgumentError::new(message))
            }
        }
    }

    #[inline]
    fn validate_if_present<F>(self, _name: &str, validator: F) -> ArgumentResult<Option<T>>
    where
        F: FnOnce(&T) -> ArgumentResult<T>,
    {
        match self {
            None => Ok(None),
            Some(value) => validator(&value).map(Some),
        }
    }
}

/// Validate that Option is None or satisfies condition
///
/// If Option is Some, validates its value; if None, passes validation.
///
/// # Parameters
///
/// * `name` - Parameter name
/// * `value` - Option value to validate
/// * `predicate` - Validation condition function
/// * `error_msg` - Error message when condition is not satisfied
///
/// # Returns
///
/// Returns `Ok(value)` if None or condition is satisfied, otherwise returns an error
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::require_null_or;
///
/// let value: Option<i32> = Some(10);
/// let result = require_null_or("value", value, |&v| v > 0, "Must be positive");
/// assert!(result.is_ok());
///
/// let none_value: Option<i32> = None;
/// let result = require_null_or("value", none_value, |&v| v > 0, "Must be positive");
/// assert!(result.is_ok());
/// ```
///
///
#[inline]
pub fn require_null_or<T, F>(
    name: &str,
    value: Option<T>,
    predicate: F,
    error_msg: &str,
) -> ArgumentResult<Option<T>>
where
    F: FnOnce(&T) -> bool,
{
    match value {
        None => Ok(None),
        Some(ref v) => {
            if !predicate(v) {
                return Err(ArgumentError::new(format!(
                    "Parameter '{}' {}",
                    name, error_msg
                )));
            }
            Ok(value)
        }
    }
}
