/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
use qubit_argument::{
    CollectionArgument,
    require_element_non_null,
};

#[test]
fn non_empty_and_length_checks_slice() {
    let v = [1, 2, 3];
    assert!(v.require_non_empty("v").is_ok());
    let empty: [i32; 0] = [];
    assert!(empty.require_non_empty("v").is_err());

    assert!(v.require_length_be("v", 3).is_ok());
    assert!(v.require_length_be("v", 2).is_err());

    assert!(v.require_length_at_least("v", 3).is_ok());
    assert!(v.require_length_at_least("v", 4).is_err());

    assert!(v.require_length_at_most("v", 3).is_ok());
    assert!(v.require_length_at_most("v", 2).is_err());

    assert!(v.require_length_in_range("v", 1, 3).is_ok());
    assert!(v.require_length_in_range("v", 4, 5).is_err());
    assert!(v.require_length_in_range("v", 3, 2).is_err());
}

#[test]
fn non_empty_and_length_checks_vec() {
    let v = vec![1, 2, 3, 4];
    assert!(v.require_non_empty("v").is_ok());
    let empty: Vec<i32> = vec![];
    assert!(empty.require_non_empty("v").is_err());

    assert!(vec![1, 2, 3].require_length_be("v", 3).is_ok());
    assert!(vec![1, 2, 3].require_length_be("v", 2).is_err());

    assert!(vec![1, 2, 3].require_length_at_least("v", 2).is_ok());
    assert!(vec![1, 2, 3].require_length_at_least("v", 4).is_err());

    assert!(vec![1, 2, 3].require_length_at_most("v", 3).is_ok());
    assert!(vec![1, 2, 3].require_length_at_most("v", 2).is_err());

    assert!(vec![1, 2, 3].require_length_in_range("v", 1, 3).is_ok());
    assert!(vec![1, 2, 3].require_length_in_range("v", 4, 5).is_err());
    assert!(vec![1, 2, 3].require_length_in_range("v", 3, 2).is_err());
}

#[test]
fn require_element_non_null_checks() {
    let all_some = vec![Some(1), Some(2), Some(3)];
    assert!(require_element_non_null("items", &all_some).is_ok());

    let with_none = vec![Some(1), None, Some(3)];
    let err = require_element_non_null("items", &with_none).unwrap_err();
    assert!(err.message().contains("element at index 1"));

    let empty: Vec<Option<i32>> = vec![];
    assert!(require_element_non_null("items", &empty).is_ok());

    let none_first = vec![None, Some(2)];
    let err2 = require_element_non_null("items", &none_first).unwrap_err();
    assert!(err2.message().contains("element at index 0"));
}

#[test]
fn vec_impl_returns_same_reference_after_validation() {
    let values = vec![1, 2, 3];
    let validated = values.require_non_empty("values").unwrap();
    assert!(std::ptr::eq(validated, &values));

    let validated2 = values.require_length_in_range("values", 1, 3).unwrap();
    assert!(std::ptr::eq(validated2, &values));
}
