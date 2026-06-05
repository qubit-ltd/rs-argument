// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Numeric Argument Validation
//!
//! Provides validation functionality for numeric type arguments.

use super::argument_error::{
    ArgumentError,
    ArgumentResult,
};
use std::fmt::Display;

/// Internal trait that restricts `NumericArgument` to actual numeric types.
trait NumericValue: PartialOrd + Display + Copy {
    /// Returns the additive identity (zero) of the numeric type.
    fn zero() -> Self;

    /// Rejects NaN values when the numeric type supports NaN.
    ///
    /// Non-floating-point types always pass this check.
    #[inline]
    fn reject_nan(self, _name: &str) -> ArgumentResult<()> {
        Ok(())
    }
}

macro_rules! impl_numeric_value_for_int {
    ($($t:ty),+ $(,)?) => {
        $(
            impl NumericValue for $t {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }
        )+
    };
}

impl_numeric_value_for_int!(i8, i16, i32, i64, i128, isize);
impl_numeric_value_for_int!(u8, u16, u32, u64, u128, usize);

impl NumericValue for f32 {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn reject_nan(self, name: &str) -> ArgumentResult<()> {
        if self.is_nan() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must not be NaN",
                name
            )));
        }
        Ok(())
    }
}

impl NumericValue for f64 {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn reject_nan(self, name: &str) -> ArgumentResult<()> {
        if self.is_nan() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must not be NaN",
                name
            )));
        }
        Ok(())
    }
}

/// Numeric argument validation trait
///
/// Provides validation methods for all sortable numeric types, supporting
/// method chaining.
///
/// # Features
///
/// - Zero-cost abstraction: Same performance as manual checks after compilation
/// - Method chaining: Can perform multiple validations in sequence
/// - Type safety: Leverages Rust's type system to ensure correctness
/// - Clear errors: Provides friendly error messages
///
/// # Use Cases
///
/// - Validating function parameter validity
/// - Configuration value range checking
/// - User input numeric validation
///
/// # Examples
///
/// Basic usage (returns `ArgumentResult`):
///
/// ```rust
/// use qubit_argument::argument::{NumericArgument, ArgumentResult};
///
/// fn set_volume(volume: i32) -> ArgumentResult<()> {
///     let volume = volume.require_in_closed_range("volume", 0, 100)?;
///     println!("Volume: {}", volume);
///     Ok(())
/// }
/// ```
///
/// Converting to other error types:
///
/// ```rust
/// use qubit_argument::argument::NumericArgument;
///
/// fn set_volume(volume: i32) -> Result<(), String> {
///     let volume = volume
///         .require_non_negative("volume")
///         .and_then(|v| v.require_in_closed_range("volume", 0, 100))
///         .map_err(|e| e.to_string())?;
///     println!("Volume: {}", volume);
///     Ok(())
/// }
/// ```
pub trait NumericArgument: Sized {
    /// Validate that value is zero
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name for error message generation
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is zero, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value: i32 = 0;
    /// assert!(value.require_zero("value").is_ok());
    ///
    /// let non_zero: i32 = 5;
    /// assert!(non_zero.require_zero("value").is_err());
    /// ```
    fn require_zero(self, name: &str) -> ArgumentResult<Self>;

    /// Validate that value is non-zero
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is non-zero, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value: i32 = 10;
    /// assert!(value.require_non_zero("value").is_ok());
    ///
    /// let zero: i32 = 0;
    /// assert!(zero.require_non_zero("value").is_err());
    /// ```
    fn require_non_zero(self, name: &str) -> ArgumentResult<Self>;

    /// Validate that value is positive
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is greater than zero, otherwise returns an
    /// error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value: i32 = 10;
    /// assert!(value.require_positive("value").is_ok());
    ///
    /// let zero: i32 = 0;
    /// assert!(zero.require_positive("value").is_err());
    /// ```
    fn require_positive(self, name: &str) -> ArgumentResult<Self>;

    /// Validate that value is non-negative
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is non-negative, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value: i32 = 0;
    /// assert!(value.require_non_negative("value").is_ok());
    ///
    /// let negative: i32 = -5;
    /// assert!(negative.require_non_negative("value").is_err());
    /// ```
    fn require_non_negative(self, name: &str) -> ArgumentResult<Self>;

    /// Validate that value is negative
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is less than zero, otherwise returns an
    /// error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value: i32 = -5;
    /// assert!(value.require_negative("value").is_ok());
    ///
    /// let positive: i32 = 5;
    /// assert!(positive.require_negative("value").is_err());
    /// ```
    fn require_negative(self, name: &str) -> ArgumentResult<Self>;

    /// Validate that value is non-positive
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is less than or equal to zero, otherwise
    /// returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value: i32 = 0;
    /// assert!(value.require_non_positive("value").is_ok());
    ///
    /// let negative: i32 = -5;
    /// assert!(negative.require_non_positive("value").is_ok());
    ///
    /// let positive: i32 = 5;
    /// assert!(positive.require_non_positive("value").is_err());
    /// ```
    fn require_non_positive(self, name: &str) -> ArgumentResult<Self>;

    /// Validate that value is within closed interval
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min` - Minimum value (inclusive)
    /// * `max` - Maximum value (inclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is within [min, max] range, otherwise
    /// returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 50;
    /// assert!(value.require_in_closed_range("value", 0, 100).is_ok());
    ///
    /// let out_of_range = 150;
    /// assert!(out_of_range.require_in_closed_range("value", 0, 100).is_err());
    /// ```
    fn require_in_closed_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self>;

    /// Validate that value is within open interval
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min` - Minimum value (exclusive)
    /// * `max` - Maximum value (exclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is within (min, max) range, otherwise
    /// returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 50;
    /// assert!(value.require_in_open_range("value", 0, 100).is_ok());
    ///
    /// let boundary = 0;
    /// assert!(boundary.require_in_open_range("value", 0, 100).is_err());
    /// ```
    fn require_in_open_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self>;

    /// Validate that value is within left-open right-closed interval
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min` - Minimum value (exclusive)
    /// * `max` - Maximum value (inclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is within (min, max] range, otherwise
    /// returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 100;
    /// assert!(value.require_in_left_open_range("value", 0, 100).is_ok());
    ///
    /// let min_boundary = 0;
    /// assert!(min_boundary.require_in_left_open_range("value", 0, 100).is_err());
    /// ```
    fn require_in_left_open_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self>;

    /// Validate that value is within left-closed right-open interval
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min` - Minimum value (inclusive)
    /// * `max` - Maximum value (exclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is within [min, max) range, otherwise
    /// returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 0;
    /// assert!(value.require_in_right_open_range("value", 0, 100).is_ok());
    ///
    /// let max_boundary = 100;
    /// assert!(max_boundary.require_in_right_open_range("value", 0, 100).is_err());
    /// ```
    fn require_in_right_open_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self>;

    /// Validate that value is less than specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `max` - Maximum value (exclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is less than max, otherwise returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 50;
    /// assert!(value.require_less("value", 100).is_ok());
    ///
    /// let boundary = 100;
    /// assert!(boundary.require_less("value", 100).is_err());
    /// ```
    fn require_less(self, name: &str, max: Self) -> ArgumentResult<Self>;

    /// Validate that value is less than or equal to specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `max` - Maximum value (inclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is less than or equal to max, otherwise
    /// returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 100;
    /// assert!(value.require_less_equal("value", 100).is_ok());
    ///
    /// let over = 101;
    /// assert!(over.require_less_equal("value", 100).is_err());
    /// ```
    fn require_less_equal(self, name: &str, max: Self) -> ArgumentResult<Self>;

    /// Validate that value is greater than specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min` - Minimum value (exclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is greater than min, otherwise returns an
    /// error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 50;
    /// assert!(value.require_greater("value", 0).is_ok());
    ///
    /// let boundary = 0;
    /// assert!(boundary.require_greater("value", 0).is_err());
    /// ```
    fn require_greater(self, name: &str, min: Self) -> ArgumentResult<Self>;

    /// Validate that value is greater than or equal to specified value
    ///
    /// # Parameters
    ///
    /// * `name` - Parameter name
    /// * `min` - Minimum value (inclusive)
    ///
    /// # Returns
    ///
    /// Returns `Ok(self)` if value is greater than or equal to min, otherwise
    /// returns an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_argument::argument::NumericArgument;
    ///
    /// let value = 0;
    /// assert!(value.require_greater_equal("value", 0).is_ok());
    ///
    /// let under = -1;
    /// assert!(under.require_greater_equal("value", 0).is_err());
    /// ```
    fn require_greater_equal(
        self,
        name: &str,
        min: Self,
    ) -> ArgumentResult<Self>;
}

/// Implement numeric argument validation for all ordered displayable types
///
/// Automatically provides validation functionality for all standard numeric
/// types: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
/// f32, and f64.
impl<T> NumericArgument for T
where
    T: NumericValue,
{
    #[inline]
    fn require_zero(self, name: &str) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        if self != T::zero() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be zero but was: {}",
                name, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_non_zero(self, name: &str) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        if self == T::zero() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' cannot be zero",
                name
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_positive(self, name: &str) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        if self <= T::zero() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be positive but was: {}",
                name, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_non_negative(self, name: &str) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        if self < T::zero() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be non-negative but was: {}",
                name, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_negative(self, name: &str) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        if self >= T::zero() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be negative but was: {}",
                name, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_non_positive(self, name: &str) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        if self > T::zero() {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be non-positive but was: {}",
                name, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_in_closed_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        min.reject_nan("min")?;
        max.reject_nan("max")?;
        if min > max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' has invalid range: min {} is greater than max {}",
                name, min, max
            )));
        }
        if self < min || self > max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be in range [{}, {}] but was: {}",
                name, min, max, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_in_open_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        min.reject_nan("min")?;
        max.reject_nan("max")?;
        if min >= max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' has invalid range: min {} must be less than max {}",
                name, min, max
            )));
        }
        if self <= min || self >= max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be in range ({}, {}) but was: {}",
                name, min, max, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_in_left_open_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        min.reject_nan("min")?;
        max.reject_nan("max")?;
        if min > max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' has invalid range: min {} is greater than max {}",
                name, min, max
            )));
        }
        if self <= min || self > max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be in range ({}, {}] but was: {}",
                name, min, max, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_in_right_open_range(
        self,
        name: &str,
        min: Self,
        max: Self,
    ) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        min.reject_nan("min")?;
        max.reject_nan("max")?;
        if min > max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' has invalid range: min {} is greater than max {}",
                name, min, max
            )));
        }
        if self < min || self >= max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be in range [{}, {}) but was: {}",
                name, min, max, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_less(self, name: &str, max: Self) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        max.reject_nan("max")?;
        if self >= max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be less than {} but was: {}",
                name, max, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_less_equal(self, name: &str, max: Self) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        max.reject_nan("max")?;
        if self > max {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be less than or equal to {} but was: {}",
                name, max, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_greater(self, name: &str, min: Self) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        min.reject_nan("min")?;
        if self <= min {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be greater than {} but was: {}",
                name, min, self
            )));
        }
        Ok(self)
    }

    #[inline]
    fn require_greater_equal(
        self,
        name: &str,
        min: Self,
    ) -> ArgumentResult<Self> {
        self.reject_nan(name)?;
        min.reject_nan("min")?;
        if self < min {
            return Err(ArgumentError::new(format!(
                "Parameter '{}' must be greater than or equal to {} but was: {}",
                name, min, self
            )));
        }
        Ok(self)
    }
}

/// Comparison argument validation
///
/// Provides comparison validation functionality between two arguments.
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::require_equal;
///
/// let result = require_equal("width", 100, "height", 100);
/// assert!(result.is_ok());
///
/// let result = require_equal("width", 100, "height", 200);
/// assert!(result.is_err());
/// ```
#[inline]
pub fn require_equal<T>(
    name1: &str,
    value1: T,
    name2: &str,
    value2: T,
) -> ArgumentResult<()>
where
    T: PartialEq + Display,
{
    if value1 != value2 {
        return Err(ArgumentError::new(format!(
            "Parameter '{}' ({}) must equal parameter '{}' ({})",
            name1, value1, name2, value2
        )));
    }
    Ok(())
}

/// Validate that two arguments are not equal
///
/// # Parameters
///
/// * `name1` - First parameter name
/// * `value1` - First parameter value
/// * `name2` - Second parameter name
/// * `value2` - Second parameter value
///
/// # Examples
///
/// ```rust
/// use qubit_argument::argument::require_not_equal;
///
/// let result = require_not_equal("min", 0, "max", 100);
/// assert!(result.is_ok());
/// ```
#[inline]
pub fn require_not_equal<T>(
    name1: &str,
    value1: T,
    name2: &str,
    value2: T,
) -> ArgumentResult<()>
where
    T: PartialEq + Display,
{
    if value1 == value2 {
        return Err(ArgumentError::new(format!(
            "Parameters '{}' and '{}' cannot be equal (both are: {})",
            name1, name2, value1
        )));
    }
    Ok(())
}
