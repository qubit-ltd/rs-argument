// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Bounds and custom predicate validation tests.

use qubit_argument::{
    ArgumentErrorKind, IndexRole, check_bounds, check_element_index, check_position_index,
    check_position_range, require_that,
};

/// Verifies that successful custom validation returns the original value.
#[test]
fn test_require_that_returns_original_value() {
    let value = require_that(
        4_u32,
        "workers",
        |value| *value % 2 == 0,
        "even",
        "must be even",
    )
    .expect("four is even");
    assert_eq!(value, 4);
}

/// Verifies that failed custom validation preserves the supplied error data.
#[test]
fn test_require_that_reports_custom_failure() {
    let error = require_that(
        3_u32,
        "workers",
        |value| *value % 2 == 0,
        "even",
        "must be even",
    )
    .expect_err("three is not even");
    assert_eq!(error.path().as_str(), "workers");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Custom {
            code: String::from("even"),
            message: String::from("must be even"),
        },
    );
}

/// Verifies that regions wholly contained in a buffer are accepted.
#[test]
fn test_check_bounds_accepts_valid_regions() {
    assert!(check_bounds("buffer", 0, 0, 0).is_ok());
    assert!(check_bounds("buffer", 10, 20, 100).is_ok());
    assert!(check_bounds("buffer", 100, 0, 100).is_ok());
}

/// Verifies that an offset after the total length reports structured bounds.
#[test]
fn test_check_bounds_rejects_offset_after_total_length() {
    let error = check_bounds("buffer", 6, 0, 5).expect_err("offset after the buffer must fail");
    assert_eq!(error.path().as_str(), "buffer");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Bounds {
            offset: 6,
            length: 0,
            total_length: 5,
        },
    );
}

/// Verifies that a length exceeding the remainder reports structured bounds.
#[test]
fn test_check_bounds_rejects_length_after_remaining_region() {
    let error =
        check_bounds("buffer", 3, 3, 5).expect_err("region extending after the buffer must fail");
    assert_eq!(error.path().as_str(), "buffer");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Bounds {
            offset: 3,
            length: 3,
            total_length: 5,
        },
    );
}

/// Verifies that bounds validation detects overflow without unchecked addition.
#[test]
fn test_check_bounds_rejects_overflow_without_adding() {
    let error = check_bounds("buffer", usize::MAX, 1, usize::MAX)
        .expect_err("one byte cannot follow the final position");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Bounds {
            offset: usize::MAX,
            length: 1,
            total_length: usize::MAX,
        },
    );
}

/// Verifies that an element index strictly before the size is returned.
#[test]
fn test_check_element_index_returns_valid_index() {
    assert_eq!(
        check_element_index("items", 4, 5).expect("index names an element"),
        4,
    );
}

/// Verifies that an invalid element index carries the element role.
#[test]
fn test_check_element_index_reports_element_role() {
    let error = check_element_index("items", 5, 5).expect_err("the size is not an element index");
    assert_eq!(error.path().as_str(), "items");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Index {
            index: 5,
            size: 5,
            role: IndexRole::Element,
        },
    );
}

/// Verifies that a position index may equal the collection size.
#[test]
fn test_check_position_index_returns_valid_index() {
    assert_eq!(
        check_position_index("items", 5, 5).expect("the position after the final element is valid"),
        5,
    );
    assert_eq!(
        check_position_index("items", 0, 0).expect("an empty collection has one position"),
        0,
    );
}

/// Verifies that an invalid position index carries the position role.
#[test]
fn test_check_position_index_reports_position_role() {
    let error = check_position_index("items", 6, 5)
        .expect_err("position after the collection boundary must fail");
    assert_eq!(error.path().as_str(), "items");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Index {
            index: 6,
            size: 5,
            role: IndexRole::Position,
        },
    );
}

/// Verifies that position-range validation returns the validated range.
#[test]
fn test_check_position_range_returns_validated_range() {
    let range = check_position_range("items", 2, 5, 8).expect("range is valid");
    assert_eq!(range, 2..5);
}

/// Verifies that empty position ranges remain valid at both boundaries.
#[test]
fn test_check_position_range_accepts_empty_ranges() {
    assert_eq!(
        check_position_range("items", 0, 0, 5).expect("empty range at the start is valid"),
        0..0,
    );
    assert_eq!(
        check_position_range("items", 5, 5, 5).expect("empty range at the end is valid"),
        5..5,
    );
}

/// Verifies that a reversed position range reports its exact endpoints.
#[test]
fn test_check_position_range_rejects_start_after_end() {
    let error =
        check_position_range("items", 4, 3, 8).expect_err("range start must not follow its end");
    assert_eq!(error.path().as_str(), "items");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::IndexRange {
            start: 4,
            end: 3,
            size: 8,
        },
    );
}

/// Verifies that a position range ending after the size reports its bounds.
#[test]
fn test_check_position_range_rejects_end_after_size() {
    let error = check_position_range("items", 3, 9, 8)
        .expect_err("range end must not exceed the collection size");
    assert_eq!(error.path().as_str(), "items");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::IndexRange {
            start: 3,
            end: 9,
            size: 8,
        },
    );
}
