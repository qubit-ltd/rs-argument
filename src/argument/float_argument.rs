// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Ownership-preserving validation for floating-point arguments.

use crate::argument::ArgumentError;
use crate::argument::ArgumentErrorKind;
use crate::argument::ArgumentResult;
use crate::argument::ArgumentValue;
use crate::argument::internal::Sealed;

/// Validates properties specific to floating-point arguments.
///
/// This trait is implemented only for `f32` and `f64`. Successful validation
/// returns the original value with its exact bit pattern unchanged.
///
/// The trait is sealed: downstream crates can use its methods but cannot add
/// implementations for other types.
///
/// ```compile_fail
/// use qubit_argument::{ArgumentResult, FloatArgument};
///
/// struct CustomFloat;
///
/// impl FloatArgument for CustomFloat {
///     fn require_finite(self, _path: &str) -> ArgumentResult<Self> {
///         Ok(self)
///     }
/// }
/// ```
pub trait FloatArgument: Sealed + Sized {
    /// Requires this floating-point value to be finite.
    ///
    /// Success returns the original value. NaN returns
    /// [`ArgumentErrorKind::NotANumber`]; positive or negative infinity returns
    /// [`ArgumentErrorKind::NotFinite`] with the exact floating-point value at
    /// `path`.
    fn require_finite(self, path: &str) -> ArgumentResult<Self>;
}

macro_rules! impl_float_argument {
    ($($float_type:ty),+ $(,)?) => {
        $(
            impl FloatArgument for $float_type {
                /// Requires a finite value and preserves its exact bit pattern.
                #[inline]
                fn require_finite(self, path: &str) -> ArgumentResult<Self> {
                    if self.is_nan() {
                        Err(ArgumentError::new(
                            path,
                            ArgumentErrorKind::NotANumber,
                        ))
                    } else if self.is_finite() {
                        Ok(self)
                    } else {
                        Err(ArgumentError::new(
                            path,
                            ArgumentErrorKind::NotFinite {
                                actual: ArgumentValue::from(self),
                            },
                        ))
                    }
                }
            }
        )+
    };
}

impl_float_argument!(f32, f64);
