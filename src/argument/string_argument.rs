/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # String Argument Validation
//!
//! Provides validation functionality for string type arguments.
//!

use super::argument_error::{
    ArgumentError,
    ArgumentResult,
};
use regex::Regex;

/// String argument validation trait
///
/// Provides length, content, and format validation functionality for string types.
///
/// # Features
///
/// - Length validation support
/// - Blank checking support
/// - Regular expression matching support
/// - Method chaining support
///
/// # Use Cases
///
/// - User input validation
/// - Configuration parameter checking
/// - Text content validation
///
///
/// # Examples
///
/// Basic usage (returns `ArgumentResult`):
///
/// ```rust
/// use qubit_argument::argument::{StringArgument, ArgumentResult};
///
/// fn set_username(username: &str) -> ArgumentResult<()> {
///     let username = username
///         .require_non_blank("username")?
///         .require_length_in_range("username", 3, 20)?;
///     println!("Username: {}", username);
///     Ok(())
/// }
/// ```
///
/// Converting to other error types:
///
/// ```rust
/// use qubit_argument::argument::StringArgument;
///
/// fn set_username(username: &str) -> Result<(), String> {
///     let username = username
///         .require_non_blank("username")
///         .and_then(|u| u.require_length_in_range("username", 3, 20))
///         .map_err(|e| e.to_string())?;
///     println!("Username: {}", username);
///     Ok(())
/// }
/// ```
pub trait StringArgument {
    /// Validate that string is not blank
    ///
    /// Checks if the string is empty or contains only whitespace characters.
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if string is not blank, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::StringArgument;
    ///
    /// let text = "hello";
    /// assert!(text.require_non_blank("text").is_ok());
    ///
    /// let blank = "   ";
    /// assert!(blank.require_non_blank("text").is_err());
    /// ```
    fn require_non_blank(&self, name: &str) -> ArgumentResult<&Self>;

    /// Validate that string length equals the specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `length` - Expected length
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if length matches, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::StringArgument;
    ///
    /// let code = "ABC12";
    /// assert!(code.require_length_be("code", 5).is_ok());
    ///
    /// let wrong_length = "ABC";
    /// assert!(wrong_length.require_length_be("code", 5).is_err());
    /// ```
    fn require_length_be(&self, name: &str, length: usize) -> ArgumentResult<&Self>;

    /// Validate that string length is at least the specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min_length` - Minimum length
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if length is not less than minimum, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::StringArgument;
    ///
    /// let password = "secret123";
    /// assert!(password.require_length_at_least("password", 8).is_ok());
    /// ```
    fn require_length_at_least(&self, name: &str, min_length: usize) -> ArgumentResult<&Self>;

    /// Validate that string length is at most the specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `max_length` - Maximum length
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if length is not greater than maximum, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::StringArgument;
    ///
    /// let description = "Short text";
    /// assert!(description.require_length_at_most("description", 100).is_ok());
    /// ```
    fn require_length_at_most(&self, name: &str, max_length: usize) -> ArgumentResult<&Self>;

    /// Validate that string length is within the specified range
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min_length` - Minimum length (inclusive)
    /// * `max_length` - Maximum length (inclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if length is within range, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::StringArgument;
    ///
    /// let username = "alice";
    /// assert!(username.require_length_in_range("username", 3, 20).is_ok());
    /// ```
    fn require_length_in_range(
        &self,
        name: &str,
        min_length: usize,
        max_length: usize,
    ) -> ArgumentResult<&Self>;

    /// Validate that string matches regular expression
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `pattern` - Regular expression
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if matches, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::StringArgument;
    /// use regex::Regex;
    ///
    /// let email = "user@example.com";
    /// let pattern = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    /// assert!(email.require_match("email", &pattern).is_ok());
    /// ```
    fn require_match(&self, name: &str, pattern: &Regex) -> ArgumentResult<&Self>;

    /// Validate that string does not match regular expression
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `pattern` - Regular expression
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if does not match, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::StringArgument;
    /// use regex::Regex;
    ///
    /// let text = "hello world";
    /// let pattern = Regex::new(r"\d+").unwrap();
    /// assert!(text.require_not_match("text", &pattern).is_ok());
    /// ```
    fn require_not_match(&self, name: &str, pattern: &Regex) -> ArgumentResult<&Self>;
}

impl StringArgument for str {
    #[inline]
    fn require_non_blank(&self, name: &str) -> ArgumentResult<&Self> {
        if self.trim().is_empty() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' cannot be empty or contain only whitespace characters",
                name
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_length_be(&self, name: &str, length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length != length {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' length must be {} but was {}",
                name, length, actual_length
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_length_at_least(&self, name: &str, min_length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length < min_length {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' length must be at least {} but was {}",
                name, min_length, actual_length
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_length_at_most(&self, name: &str, max_length: usize) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length > max_length {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' length must be at most {} but was {}",
                name, max_length, actual_length
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_length_in_range(
        &self,
        name: &str,
        min_length: usize,
        max_length: usize,
    ) -> ArgumentResult<&Self> {
        let actual_length = self.len();
        if actual_length < min_length || actual_length > max_length {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' length must be in range [{}, {}] but was {}",
                name, min_length, max_length, actual_length
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_match(&self, name: &str, pattern: &Regex) -> ArgumentResult<&Self> {
        if !pattern.is_match(self) {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must match pattern '{}'",
                name,
                pattern.as_str()
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_not_match(&self, name: &str, pattern: &Regex) -> ArgumentResult<&Self> {
        if pattern.is_match(self) {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' cannot match pattern '{}'",
                name,
                pattern.as_str()
            )));
        }
        Ok(self)
    }
}

impl StringArgument for String {
    #[inline]
    fn require_non_blank(&self, name: &str) -> ArgumentResult<&Self> {
        self.as_str().require_non_blank(name).map(|_| self)
    }

    #[inline]
    fn require_length_be(&self, name: &str, length: usize) -> ArgumentResult<&Self> {
        self.as_str().require_length_be(name, length).map(|_| self)
    }

    #[inline]
    fn require_length_at_least(&self, name: &str, min_length: usize) -> ArgumentResult<&Self> {
        self.as_str()
            .require_length_at_least(name, min_length)
            .map(|_| self)
    }

    #[inline]
    fn require_length_at_most(&self, name: &str, max_length: usize) -> ArgumentResult<&Self> {
        self.as_str()
            .require_length_at_most(name, max_length)
            .map(|_| self)
    }

    #[inline]
    fn require_length_in_range(
        &self,
        name: &str,
        min_length: usize,
        max_length: usize,
    ) -> ArgumentResult<&Self> {
        self.as_str()
            .require_length_in_range(name, min_length, max_length)
            .map(|_| self)
    }

    #[inline]
    fn require_match(&self, name: &str, pattern: &Regex) -> ArgumentResult<&Self> {
        self.as_str().require_match(name, pattern).map(|_| self)
    }

    #[inline]
    fn require_not_match(&self, name: &str, pattern: &Regex) -> ArgumentResult<&Self> {
        self.as_str().require_not_match(name, pattern).map(|_| self)
    }
}
