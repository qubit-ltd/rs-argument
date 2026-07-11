// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for captured argument values.

use std::time::Duration;

use qubit_argument::ArgumentValue;

/// Verifies conversions from every signed primitive integer type.
#[test]
fn test_from_preserves_signed_integers() {
    assert_eq!(ArgumentValue::from(i8::MIN), ArgumentValue::Signed(-128));
    assert_eq!(
        ArgumentValue::from(i16::MIN),
        ArgumentValue::Signed(-32_768)
    );
    assert_eq!(
        ArgumentValue::from(i32::MIN),
        ArgumentValue::Signed(-2_147_483_648)
    );
    assert_eq!(
        ArgumentValue::from(i64::MIN),
        ArgumentValue::Signed(i128::from(i64::MIN))
    );
    assert_eq!(
        ArgumentValue::from(i128::MIN),
        ArgumentValue::Signed(i128::MIN)
    );
    assert_eq!(
        ArgumentValue::from(isize::MIN),
        ArgumentValue::Signed(isize::MIN as i128)
    );
}

/// Verifies conversions from every unsigned primitive integer type.
#[test]
fn test_from_preserves_unsigned_integers() {
    assert_eq!(ArgumentValue::from(u8::MAX), ArgumentValue::Unsigned(255));
    assert_eq!(
        ArgumentValue::from(u16::MAX),
        ArgumentValue::Unsigned(65_535)
    );
    assert_eq!(
        ArgumentValue::from(u32::MAX),
        ArgumentValue::Unsigned(4_294_967_295)
    );
    assert_eq!(
        ArgumentValue::from(u64::MAX),
        ArgumentValue::Unsigned(u128::from(u64::MAX))
    );
    assert_eq!(
        ArgumentValue::from(u128::MAX),
        ArgumentValue::Unsigned(u128::MAX)
    );
    assert_eq!(
        ArgumentValue::from(usize::MAX),
        ArgumentValue::Unsigned(usize::MAX as u128)
    );
}

/// Verifies that floating-point values retain their exact bit patterns.
#[test]
fn test_argument_value_preserves_float_bits() {
    let negative_zero = ArgumentValue::from(-0.0_f64);
    let positive_zero = ArgumentValue::from(0.0_f64);
    assert_ne!(negative_zero, positive_zero);
    assert_eq!(negative_zero.to_string(), "-0");
}

/// Verifies that the `f32` conversion stores the source bit pattern.
#[test]
fn test_from_preserves_float32_bits() {
    let value = -0.0_f32;
    assert_eq!(
        ArgumentValue::from(value),
        ArgumentValue::Float32(value.to_bits())
    );
}

/// Verifies that debug formatting reconstructs special floating-point values.
#[test]
fn test_fmt_debug_reconstructs_special_floats() {
    assert_eq!(
        format!("{:?}", ArgumentValue::from(-0.0_f32)),
        "Float32(-0.0)"
    );
    assert_eq!(
        format!("{:?}", ArgumentValue::from(f64::INFINITY)),
        "Float64(inf)"
    );
    assert_eq!(
        format!("{:?}", ArgumentValue::from(f64::NAN)),
        "Float64(NaN)"
    );
}

/// Verifies that display formatting reconstructs special floating-point values.
#[test]
fn test_fmt_display_reconstructs_special_floats() {
    assert_eq!(ArgumentValue::from(f32::NEG_INFINITY).to_string(), "-inf");
    assert_eq!(ArgumentValue::from(f64::NAN).to_string(), "NaN");
}

/// Verifies debug and display formatting for signed and unsigned integers.
#[test]
fn test_fmt_formats_integer_variants() {
    let signed = ArgumentValue::from(-7_i32);
    assert_eq!(format!("{signed:?}"), "Signed(-7)");
    assert_eq!(signed.to_string(), "-7");

    let unsigned = ArgumentValue::from(7_u32);
    assert_eq!(format!("{unsigned:?}"), "Unsigned(7)");
    assert_eq!(unsigned.to_string(), "7");
}

/// Verifies that duration values retain their exact seconds and nanoseconds.
#[test]
fn test_from_preserves_duration() {
    let duration = Duration::new(12, 345);
    assert_eq!(
        ArgumentValue::from(duration),
        ArgumentValue::Duration(duration),
    );
}

/// Verifies that duration diagnostics retain their unit-bearing representation.
#[test]
fn test_fmt_formats_duration() {
    let duration = Duration::from_millis(1_500);
    let value = ArgumentValue::from(duration);
    assert_eq!(format!("{value:?}"), "Duration(1.5s)");
    assert_eq!(value.to_string(), "1.5s");
}
