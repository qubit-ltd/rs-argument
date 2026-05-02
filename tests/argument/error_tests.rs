/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
use qubit_argument::{
    ArgumentError,
    ArgumentResult,
};

#[test]
fn argument_error_new_and_message() {
    let e = ArgumentError::new("Invalid parameter");
    assert_eq!(e.message(), "Invalid parameter");
}

#[test]
fn argument_error_display() {
    let e = ArgumentError::new("Display message");
    assert_eq!(format!("{}", e), "Display message");
}

#[test]
fn argument_error_from_str_and_string() {
    let e1: ArgumentError = "Error A".into();
    assert_eq!(e1.message(), "Error A");

    let e2: ArgumentError = String::from("Error B").into();
    assert_eq!(e2.message(), "Error B");
}

#[test]
fn argument_result_usage() {
    fn validate_positive(v: i32) -> ArgumentResult<i32> {
        if v > 0 {
            Ok(v)
        } else {
            Err(ArgumentError::new("Value must be positive"))
        }
    }

    assert_eq!(validate_positive(3).unwrap(), 3);
    let err = validate_positive(0).unwrap_err();
    assert!(err.message().contains("Value must be positive"));
}
