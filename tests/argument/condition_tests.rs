// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_argument::{
    check_argument,
    check_argument_fmt,
    check_argument_with_message,
    check_bounds,
    check_element_index,
    check_position_index,
    check_position_indexes,
    check_state,
    check_state_with_message,
};

#[test]
fn basic_argument_and_state_checks() {
    assert!(check_argument(true).is_ok());
    assert!(check_argument(false).is_err());
    assert!(check_state(true).is_ok());
    assert!(check_state(false).is_err());

    let err = check_argument_with_message(false, "Count must be positive")
        .unwrap_err();
    assert!(err.to_string().contains("Count must be positive"));

    let msg = format!("Value {} exceeds maximum value {}", 150, 100);
    let err2 = check_argument_fmt(false, msg.clone()).unwrap_err();
    assert_eq!(err2.to_string(), msg);

    let err3 =
        check_state_with_message(false, "Connection must be established first")
            .unwrap_err();
    assert!(
        err3.to_string()
            .contains("Connection must be established first")
    );
}

#[test]
fn bounds_checks() {
    // ok
    assert!(check_bounds(0, 0, 0).is_ok());
    assert!(check_bounds(10, 20, 100).is_ok());
    assert!(check_bounds(100, 0, 100).is_ok());

    // offset > total_length
    assert!(check_bounds(1, 0, 0).is_err());

    // length > total_length - offset
    assert!(check_bounds(90, 20, 100).is_err());
    assert!(check_bounds(100, 1, 100).is_err());
}

#[test]
fn element_index_checks() {
    assert_eq!(check_element_index(0, 1).unwrap(), 0);
    assert!(check_element_index(1, 1).is_err());
    assert!(check_element_index(0, 0).is_err());
}

#[test]
fn position_index_checks() {
    assert_eq!(check_position_index(0, 0).unwrap(), 0);
    assert_eq!(check_position_index(1, 1).unwrap(), 1);
    assert!(check_position_index(2, 1).is_err());
}

#[test]
fn position_indexes_checks() {
    assert!(check_position_indexes(0, 0, 0).is_ok());
    assert!(check_position_indexes(0, 1, 1).is_ok());
    assert!(check_position_indexes(0, 1, 0).is_err());
    assert!(check_position_indexes(2, 1, 3).is_err());
}

#[test]
fn test_check_argument_fmt_with_different_formats() {
    // Test formatted messages
    let msg1 = format!("Value {} is not in range [{}, {}]", 150, 0, 100);
    assert!(check_argument_fmt(false, msg1).is_err());

    let msg2 = format!("Parameter '{}' must be greater than {}", "age", 18);
    assert!(check_argument_fmt(true, msg2).is_ok());
}

#[test]
fn test_check_bounds_edge_cases() {
    // Test edge cases
    assert!(check_bounds(0, 0, 1).is_ok()); // Empty range
    assert!(check_bounds(0, 1, 1).is_ok()); // Full range
    assert!(check_bounds(1, 0, 1).is_ok()); // Empty range starting from middle

    // Test overflow cases
    assert!(check_bounds(usize::MAX, 1, usize::MAX).is_err());
}

#[test]
fn test_check_element_index_more_cases() {
    // Test more cases
    assert_eq!(check_element_index(0, 10).unwrap(), 0);
    assert_eq!(check_element_index(9, 10).unwrap(), 9);
    assert!(check_element_index(10, 10).is_err());
    assert!(check_element_index(100, 10).is_err());
}

#[test]
fn test_check_position_index_edge_cases() {
    // Edge cases
    assert_eq!(check_position_index(0, 10).unwrap(), 0);
    assert_eq!(check_position_index(10, 10).unwrap(), 10);
    assert!(check_position_index(11, 10).is_err());

    // Zero length
    assert_eq!(check_position_index(0, 0).unwrap(), 0);
    assert!(check_position_index(1, 0).is_err());
}

#[test]
fn test_check_position_indexes_comprehensive() {
    // Full range
    assert!(check_position_indexes(0, 10, 10).is_ok());

    // Middle range
    assert!(check_position_indexes(3, 7, 10).is_ok());

    // start > end
    assert!(check_position_indexes(5, 3, 10).is_err());

    // end > size
    assert!(check_position_indexes(5, 11, 10).is_err());

    // start > size
    assert!(check_position_indexes(11, 11, 10).is_err());
}

#[test]
fn test_state_checks_different_scenarios() {
    // Test different scenarios for state checks
    let is_connected = true;
    assert!(check_state(is_connected).is_ok());

    let is_initialized = false;
    assert!(check_state(is_initialized).is_err());

    let has_permission = false;
    let err = check_state_with_message(
        has_permission,
        "Administrator privileges required",
    )
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("Administrator privileges required")
    );
}

#[test]
fn test_all_ok_branches() {
    // Ensure all success branches are covered
    assert!(check_argument(true).is_ok());
    assert!(check_argument_with_message(true, "any").is_ok());
    assert!(check_argument_fmt(true, "any".to_string()).is_ok());
    assert!(check_state(true).is_ok());
    assert!(check_state_with_message(true, "any").is_ok());
}
