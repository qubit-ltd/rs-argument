// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Primitive numeric values supported by numeric validation.

use crate::argument::ArgumentValue;
use crate::argument::internal::Sealed;

/// Restricts numeric validation to supported primitive numeric values.
///
/// Implementations provide the type's zero value, an exact structured error
/// representation, and NaN detection. The trait is private so arbitrary
/// partially ordered caller types cannot opt into numeric validation.
pub(in crate::argument) trait NumericValue:
    Sealed + Copy + PartialOrd
{
    /// Returns the zero value for this primitive numeric type.
    ///
    /// # Returns
    ///
    /// The type's zero value.
    fn zero() -> Self;

    /// Captures this value without losing integer magnitude or floating bits.
    ///
    /// # Returns
    ///
    /// The exact structured representation of this value.
    fn to_argument_value(self) -> ArgumentValue;

    /// Returns whether this value is a floating-point NaN.
    ///
    /// Integer implementations always return `false`.
    ///
    /// # Returns
    ///
    /// `true` for a floating-point NaN; otherwise, `false`.
    fn is_nan(self) -> bool;
}

/// Implements primitive numeric conversion and non-NaN behavior for integers.
macro_rules! impl_numeric_value_for_integer {
    ($($numeric_type:ty),+ $(,)?) => {
        $(
            impl NumericValue for $numeric_type {
                /// Returns integer zero.
                ///
                /// # Returns
                ///
                /// Zero of this integer type.
                #[inline]
                fn zero() -> Self {
                    0
                }

                /// Captures the integer without losing its value.
                ///
                /// # Returns
                ///
                /// The exact structured integer value.
                #[inline(always)]
                fn to_argument_value(self) -> ArgumentValue {
                    ArgumentValue::from(self)
                }

                /// Reports that an integer can never be NaN.
                ///
                /// # Returns
                ///
                /// Always `false`.
                #[inline(always)]
                fn is_nan(self) -> bool {
                    false
                }
            }
        )+
    };
}

impl_numeric_value_for_integer!(i8, i16, i32, i64, i128, isize);
impl_numeric_value_for_integer!(u8, u16, u32, u64, u128, usize);

impl NumericValue for f32 {
    /// Returns positive floating-point zero.
    ///
    /// # Returns
    ///
    /// Positive `f32` zero.
    #[inline]
    fn zero() -> Self {
        0.0
    }

    /// Captures the exact IEEE 754 bit pattern of this value.
    ///
    /// # Returns
    ///
    /// The exact structured floating-point value.
    #[inline(always)]
    fn to_argument_value(self) -> ArgumentValue {
        ArgumentValue::from(self)
    }

    /// Returns whether this value is NaN.
    ///
    /// # Returns
    ///
    /// `true` when this value is NaN; otherwise, `false`.
    #[inline(always)]
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

impl NumericValue for f64 {
    /// Returns positive floating-point zero.
    ///
    /// # Returns
    ///
    /// Positive `f64` zero.
    #[inline]
    fn zero() -> Self {
        0.0
    }

    /// Captures the exact IEEE 754 bit pattern of this value.
    ///
    /// # Returns
    ///
    /// The exact structured floating-point value.
    #[inline(always)]
    fn to_argument_value(self) -> ArgumentValue {
        ArgumentValue::from(self)
    }

    /// Returns whether this value is NaN.
    ///
    /// # Returns
    ///
    /// `true` when this value is NaN; otherwise, `false`.
    #[inline(always)]
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}
