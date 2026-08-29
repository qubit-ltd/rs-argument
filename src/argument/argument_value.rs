// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lossless representations of scalar argument values.

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::time::Duration;

/// A scalar value captured for a validation constraint or error.
///
/// Floating-point values are stored as raw IEEE 754 bits, so equality and
/// hashing preserve distinctions such as signed zero and NaN payloads.
/// Duration values retain their exact seconds and nanoseconds. Formatting
/// reconstructs floats and preserves unit-bearing duration diagnostics.
///
/// This enum is non-exhaustive. Downstream matches must include a wildcard arm
/// so future scalar representations can be added without a breaking release.
///
/// ```compile_fail
/// use qubit_argument::ArgumentValue;
///
/// fn classify(value: ArgumentValue) -> &'static str {
///     match value {
///         ArgumentValue::Signed(_) => "signed",
///         ArgumentValue::Unsigned(_) => "unsigned",
///         ArgumentValue::Float32(_) => "f32",
///         ArgumentValue::Float64(_) => "f64",
///         ArgumentValue::Duration(_) => "duration",
///     }
/// }
/// ```
///
/// ```compile_fail
/// #![deny(unused_must_use)]
/// use qubit_argument::ArgumentValue;
///
/// ArgumentValue::Signed(1);
/// ```
#[non_exhaustive]
#[must_use]
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
    /// An exact standard-library duration value.
    Duration(Duration),
}

macro_rules! impl_from_signed_integer {
    ($($source:ty),+ $(,)?) => {
        $(
            impl From<$source> for ArgumentValue {
                /// Converts a signed primitive integer without losing its value.
                #[inline(always)]
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
                #[inline(always)]
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
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self::Float32(value.to_bits())
    }
}

impl From<f64> for ArgumentValue {
    /// Captures the exact IEEE 754 bit pattern of a 64-bit float.
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self::Float64(value.to_bits())
    }
}

impl From<Duration> for ArgumentValue {
    /// Captures an exact standard-library duration value.
    #[inline(always)]
    fn from(value: Duration) -> Self {
        Self::Duration(value)
    }
}

impl Debug for ArgumentValue {
    /// Formats the variant with its reconstructed scalar value.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signed(value) => formatter.debug_tuple("Signed").field(value).finish(),
            Self::Unsigned(value) => formatter.debug_tuple("Unsigned").field(value).finish(),
            Self::Float32(bits) => formatter.debug_tuple("Float32").field(&f32::from_bits(*bits)).finish(),
            Self::Float64(bits) => formatter.debug_tuple("Float64").field(&f64::from_bits(*bits)).finish(),
            Self::Duration(value) => formatter.debug_tuple("Duration").field(value).finish(),
        }
    }
}

impl Display for ArgumentValue {
    /// Formats the represented scalar value without a variant label.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signed(value) => Display::fmt(value, formatter),
            Self::Unsigned(value) => Display::fmt(value, formatter),
            Self::Float32(bits) => Display::fmt(&f32::from_bits(*bits), formatter),
            Self::Float64(bits) => Display::fmt(&f64::from_bits(*bits), formatter),
            Self::Duration(value) => Debug::fmt(value, formatter),
        }
    }
}
