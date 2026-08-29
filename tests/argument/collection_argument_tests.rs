// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_argument::ArgumentErrorKind;
use qubit_argument::CollectionArgument;
use qubit_argument::LengthConstraint;
use qubit_argument::LengthMetric;

/// A value that cannot be cloned, used to verify ownership-preserving APIs.
#[derive(Debug, PartialEq, Eq)]
struct NonClone(u32);

#[test]
fn test_require_non_empty_preserves_vec_ownership() {
    let values = vec![NonClone(1), NonClone(2), NonClone(3)];
    let validated: Vec<NonClone> = values.require_non_empty("values").expect("vector is non-empty");
    assert_eq!(validated, vec![NonClone(1), NonClone(2), NonClone(3)]);
}

#[test]
fn test_require_non_empty_preserves_borrowed_slice() {
    let values = [1, 2, 3];
    let slice: &[i32] = &values;
    let validated: &[i32] = slice.require_non_empty("values").expect("slice is non-empty");
    assert!(std::ptr::eq(validated, slice));
}

#[test]
fn test_require_non_empty_preserves_array_ownership() {
    let values = [NonClone(1), NonClone(2)];
    let validated: [NonClone; 2] = values.require_non_empty("values").expect("array is non-empty");
    assert_eq!(validated, [NonClone(1), NonClone(2)]);
}

#[test]
fn test_require_non_empty_reports_empty_collection() {
    let error = Vec::<i32>::new()
        .require_non_empty("values")
        .expect_err("empty vector must fail");
    assert_eq!(error.path().as_str(), "values");
    assert_eq!(error.kind(), &ArgumentErrorKind::Empty);
}

#[test]
fn test_require_len_accepts_exact_length() {
    let validated: [i32; 3] = [1, 2, 3]
        .require_len("values", 3)
        .expect("array has the required length");
    assert_eq!(validated, [1, 2, 3]);
}

#[test]
fn test_require_len_reports_exact_length_error() {
    let error = vec![1, 2]
        .require_len("values", 3)
        .expect_err("vector has the wrong length");
    assert_eq!(error.path().as_str(), "values");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Length {
            actual: 2,
            constraint: LengthConstraint::Exact(3),
            metric: LengthMetric::Elements,
        },
    );
}

#[test]
fn test_require_len_at_least_checks_minimum_length() {
    let values = [1, 2, 3];
    let slice: &[i32] = &values;
    let validated: &[i32] = slice
        .require_len_at_least("values", 3)
        .expect("slice meets the inclusive minimum");
    assert!(std::ptr::eq(validated, slice));

    let error = [1, 2]
        .require_len_at_least("values", 3)
        .expect_err("array is shorter than the minimum");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Length {
            actual: 2,
            constraint: LengthConstraint::AtLeast(3),
            metric: LengthMetric::Elements,
        },
    );
}

#[test]
fn test_require_len_at_most_checks_maximum_length() {
    let validated: Vec<i32> = vec![1, 2, 3]
        .require_len_at_most("values", 3)
        .expect("vector meets the inclusive maximum");
    assert_eq!(validated, vec![1, 2, 3]);

    let error = [1, 2, 3]
        .require_len_at_most("values", 2)
        .expect_err("array is longer than the maximum");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Length {
            actual: 3,
            constraint: LengthConstraint::AtMost(2),
            metric: LengthMetric::Elements,
        },
    );
}

#[test]
fn test_require_len_in_checks_inclusive_range() {
    let at_minimum: [i32; 2] = [1, 2]
        .require_len_in("values", 2, 3)
        .expect("minimum endpoint is included");
    assert_eq!(at_minimum, [1, 2]);

    let at_maximum: Vec<i32> = vec![1, 2, 3]
        .require_len_in("values", 2, 3)
        .expect("maximum endpoint is included");
    assert_eq!(at_maximum, vec![1, 2, 3]);

    let error = [1]
        .require_len_in("values", 2, 3)
        .expect_err("array is outside the required range");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Length {
            actual: 1,
            constraint: LengthConstraint::InRange { min: 2, max: 3 },
            metric: LengthMetric::Elements,
        },
    );
}

#[test]
fn test_require_len_in_reports_invalid_constraint() {
    let error = [1, 2]
        .require_len_in("values", 3, 1)
        .expect_err("reversed length range must fail");
    assert_eq!(error.path().as_str(), "values");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::InvalidLengthConstraint {
            constraint: LengthConstraint::InRange { min: 3, max: 1 },
            metric: LengthMetric::Elements,
        },
    );
}

/// Verifies every vector length method on both successful and failing paths.
#[test]
fn test_vec_supports_all_length_methods() {
    assert_eq!(
        vec![1, 2]
            .require_len("values", 2)
            .expect("vector has the exact length"),
        vec![1, 2],
    );
    assert_eq!(
        vec![1, 2]
            .require_len_at_least("values", 2)
            .expect("vector meets the minimum length"),
        vec![1, 2],
    );
    assert!(vec![1].require_len_at_least("values", 2).is_err());
    assert!(vec![1, 2].require_len_at_most("values", 1).is_err());
    assert!(vec![1].require_len_in("values", 2, 3).is_err());
}

/// Verifies every borrowed-slice length method and its error propagation.
#[test]
fn test_borrowed_slice_supports_all_length_methods() {
    let empty: &[i32] = &[];
    assert!(empty.require_non_empty("values").is_err());

    let values = [1, 2, 3];
    let slice: &[i32] = &values;
    assert!(std::ptr::eq(
        slice.require_len("values", 3).expect("slice has the exact length"),
        slice,
    ));
    assert!(slice.require_len("values", 2).is_err());
    assert!(slice.require_len_at_least("values", 4).is_err());
    assert!(std::ptr::eq(
        slice
            .require_len_at_most("values", 3)
            .expect("slice meets the maximum length"),
        slice,
    ));
    assert!(slice.require_len_at_most("values", 2).is_err());
    assert!(std::ptr::eq(
        slice
            .require_len_in("values", 2, 3)
            .expect("slice lies in the inclusive length range"),
        slice,
    ));
    assert!(slice.require_len_in("values", 4, 5).is_err());
}

/// Verifies the remaining empty and successful array validation paths.
#[test]
fn test_array_supports_all_collection_paths() {
    assert!([0_i32; 0].require_non_empty("values").is_err());
    assert!([1, 2].require_len("values", 1).is_err());
    assert_eq!(
        [1, 2]
            .require_len_at_least("values", 2)
            .expect("array meets the minimum length"),
        [1, 2],
    );
    assert_eq!(
        [1, 2]
            .require_len_at_most("values", 2)
            .expect("array meets the maximum length"),
        [1, 2],
    );
}
