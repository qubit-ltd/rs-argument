// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private marker for library-controlled extension traits.

use std::time::Duration;

use crate::argument::ArgumentError;

/// Prevents downstream crates from implementing validation extension traits.
pub trait Sealed {}

macro_rules! impl_sealed_for_primitive {
    ($($primitive_type:ty),+ $(,)?) => {
        $(
            impl Sealed for $primitive_type {}
        )+
    };
}

impl_sealed_for_primitive!(i8, i16, i32, i64, i128, isize);
impl_sealed_for_primitive!(u8, u16, u32, u64, u128, usize);
impl_sealed_for_primitive!(f32, f64);

impl Sealed for Duration {}

impl Sealed for String {}

impl Sealed for &str {}

impl<T> Sealed for Vec<T> {}

impl<T> Sealed for &[T] {}

impl<T, const N: usize> Sealed for [T; N] {}

impl<T> Sealed for Option<T> {}

impl<T> Sealed for Result<T, ArgumentError> {}
