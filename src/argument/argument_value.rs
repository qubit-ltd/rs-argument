// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lossless representations of primitive numeric argument values.

use std::fmt::{self, Debug, Display, Formatter};

/// A primitive numeric value captured for a validation constraint or error.
///
/// Floating-point values are stored as raw IEEE 754 bits, so equality and
/// hashing preserve distinctions such as signed zero and NaN payloads.
/// Formatting reconstructs the original float so negative zero, infinities,
/// and NaN remain visible as floating-point values.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgumentValue {
    /// A signed integer represented without loss.
    Signed(i128),
    /// An unsigned integer represented without loss.
    Unsigned(u128),
    /// The raw bits of a 32-bit floating-point value.
    Float32(u32),
    /// The raw bits of a 64-bit floating-point value.
    Float64(u64),
}

macro_rules! impl_from_signed_integer {
    ($($source:ty),+ $(,)?) => {
        $(
            impl From<$source> for ArgumentValue {
                /// Converts a signed primitive integer without losing its value.
                #[inline]
                fn from(value: $source) -> Self {
                    Self::Signed(value as i128)
                }
            }
        )+
    };
}

macro_rules! impl_from_unsigned_integer {
    ($($source:ty),+ $(,)?) => {
        $(
            impl From<$source> for ArgumentValue {
                /// Converts an unsigned primitive integer without losing its value.
                #[inline]
                fn from(value: $source) -> Self {
                    Self::Unsigned(value as u128)
                }
            }
        )+
    };
}

impl_from_signed_integer!(i8, i16, i32, i64, i128, isize);
impl_from_unsigned_integer!(u8, u16, u32, u64, u128, usize);

impl From<f32> for ArgumentValue {
    /// Captures the exact IEEE 754 bit pattern of a 32-bit float.
    #[inline]
    fn from(value: f32) -> Self {
        Self::Float32(value.to_bits())
    }
}

impl From<f64> for ArgumentValue {
    /// Captures the exact IEEE 754 bit pattern of a 64-bit float.
    #[inline]
    fn from(value: f64) -> Self {
        Self::Float64(value.to_bits())
    }
}

impl Debug for ArgumentValue {
    /// Formats the variant with a reconstructed primitive numeric value.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signed(value) => formatter.debug_tuple("Signed").field(value).finish(),
            Self::Unsigned(value) => formatter.debug_tuple("Unsigned").field(value).finish(),
            Self::Float32(bits) => formatter
                .debug_tuple("Float32")
                .field(&f32::from_bits(*bits))
                .finish(),
            Self::Float64(bits) => formatter
                .debug_tuple("Float64")
                .field(&f64::from_bits(*bits))
                .finish(),
        }
    }
}

impl Display for ArgumentValue {
    /// Formats the represented primitive value without a variant label.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signed(value) => Display::fmt(value, formatter),
            Self::Unsigned(value) => Display::fmt(value, formatter),
            Self::Float32(bits) => Display::fmt(&f32::from_bits(*bits), formatter),
            Self::Float64(bits) => Display::fmt(&f64::from_bits(*bits), formatter),
        }
    }
}
