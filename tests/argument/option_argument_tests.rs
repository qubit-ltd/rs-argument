// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for ownership-preserving `Option` argument validation.

use std::cell::Cell;

use qubit_argument::{
    ArgumentError,
    ArgumentErrorKind,
    OptionArgument,
};

/// A value that cannot be cloned, used to verify ownership-preserving APIs.
#[derive(Debug, PartialEq, Eq)]
struct NonClone(u32);

/// Validates a borrowed value, rejecting zero for coverage across one function
/// type.
fn validate_non_zero_borrowed(value: &NonClone) -> Result<(), ArgumentError> {
    if value.0 == 0 {
        Err(ArgumentError::new(
            "value",
            ArgumentErrorKind::Custom {
                code: String::from("zero"),
                message: String::from("value must not be zero"),
            },
        ))
    } else {
        Ok(())
    }
}

/// Validates an owned value, rejecting zero for coverage across one function
/// type.
fn validate_non_zero_owned(value: NonClone) -> Result<NonClone, ArgumentError> {
    if value.0 == 0 {
        Err(ArgumentError::new(
            "value",
            ArgumentErrorKind::Custom {
                code: String::from("zero"),
                message: String::from("value must not be zero"),
            },
        ))
    } else {
        Ok(value)
    }
}

#[test]
fn test_require_some_preserves_non_clone_value() {
    let value = Some(NonClone(7));
    let validated: NonClone = value
        .require_some("value")
        .expect("option contains a value");
    assert_eq!(validated, NonClone(7));
}

#[test]
fn test_require_some_reports_missing_value() {
    let error = None::<NonClone>
        .require_some("value")
        .expect_err("missing value must fail");
    assert_eq!(error.path().as_str(), "value");
    assert_eq!(error.kind(), &ArgumentErrorKind::Missing);
}

#[test]
fn test_validate_if_some_preserves_non_clone_value() {
    let value = Some(NonClone(7));
    let validated = value
        .validate_if_some(|item| {
            if item.0 == 7 {
                Ok(())
            } else {
                Err(ArgumentError::new(
                    "item",
                    ArgumentErrorKind::Custom {
                        code: String::from("unexpected_value"),
                        message: String::from("item must equal seven"),
                    },
                ))
            }
        })
        .expect("value satisfies validator");
    assert_eq!(validated, Some(NonClone(7)));
}

#[test]
fn test_validate_if_some_skips_validator_for_none() {
    let validator_called = Cell::new(false);
    let value: Option<NonClone> = None;
    let validated = value
        .validate_if_some(|_| {
            validator_called.set(true);
            Ok(())
        })
        .expect("missing optional value does not require validation");
    assert_eq!(validated, None);
    assert!(!validator_called.get());
}

#[test]
fn test_validate_if_some_executes_validator_once_for_some() {
    let call_count = Cell::new(0_u32);
    let value = Some(NonClone(9));
    let validated = value
        .validate_if_some(|item| {
            call_count.set(call_count.get() + 1);
            assert_eq!(item, &NonClone(9));
            Ok(())
        })
        .expect("value satisfies validator");
    assert_eq!(validated, Some(NonClone(9)));
    assert_eq!(call_count.get(), 1);
}

#[test]
fn test_validate_if_some_propagates_structured_error() {
    let error = Some(NonClone(1))
        .validate_if_some(|_| {
            Err(ArgumentError::new(
                "item",
                ArgumentErrorKind::Custom {
                    code: String::from("rejected"),
                    message: String::from("item was rejected"),
                },
            ))
        })
        .expect_err("validator failure must propagate");
    assert_eq!(error.path().as_str(), "item");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Custom {
            code: String::from("rejected"),
            message: String::from("item was rejected"),
        },
    );
}

#[test]
fn test_validate_if_some_covers_all_outcomes_for_one_validator_type() {
    let present = Some(NonClone(1))
        .validate_if_some(validate_non_zero_borrowed)
        .expect("non-zero value satisfies validator");
    assert_eq!(present, Some(NonClone(1)));

    let error = Some(NonClone(0))
        .validate_if_some(validate_non_zero_borrowed)
        .expect_err("zero value must fail validation");
    assert_eq!(error.path().as_str(), "value");

    let absent = None::<NonClone>
        .validate_if_some(validate_non_zero_borrowed)
        .expect("missing optional value bypasses validation");
    assert_eq!(absent, None);
}

/// Verifies owned validation can transform a non-clone value.
#[test]
fn test_validate_some_transforms_owned_non_clone_value() {
    let validated = Some(NonClone(7))
        .validate_some(|item| Ok(NonClone(item.0 + 1)))
        .expect("present value satisfies validator");
    assert_eq!(validated, Some(NonClone(8)));
}

/// Verifies `None` bypasses the owned validator.
#[test]
fn test_validate_some_skips_validator_for_none() {
    let validator_called = Cell::new(false);
    let validated = None::<NonClone>
        .validate_some(|item| {
            validator_called.set(true);
            Ok(item)
        })
        .expect("missing optional value does not require validation");
    assert_eq!(validated, None);
    assert!(!validator_called.get());
}

/// Verifies a present value invokes the owned validator exactly once.
#[test]
fn test_validate_some_executes_validator_once_for_some() {
    let call_count = Cell::new(0_u32);
    let validated = Some(NonClone(9))
        .validate_some(|item| {
            call_count.set(call_count.get() + 1);
            Ok(item)
        })
        .expect("present value satisfies validator");
    assert_eq!(validated, Some(NonClone(9)));
    assert_eq!(call_count.get(), 1);
}

/// Verifies owned validation propagates structured errors unchanged.
#[test]
fn test_validate_some_propagates_structured_error() {
    let error = Some(NonClone(1))
        .validate_some(|_| {
            Err(ArgumentError::new(
                "item",
                ArgumentErrorKind::Custom {
                    code: String::from("rejected"),
                    message: String::from("item was rejected"),
                },
            ))
        })
        .expect_err("validator failure must propagate");
    assert_eq!(error.path().as_str(), "item");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Custom {
            code: String::from("rejected"),
            message: String::from("item was rejected"),
        },
    );
}

#[test]
fn test_validate_some_covers_all_outcomes_for_one_validator_type() {
    let present = Some(NonClone(1))
        .validate_some(validate_non_zero_owned)
        .expect("non-zero value satisfies validator");
    assert_eq!(present, Some(NonClone(1)));

    let error = Some(NonClone(0))
        .validate_some(validate_non_zero_owned)
        .expect_err("zero value must fail validation");
    assert_eq!(error.path().as_str(), "value");

    let absent = None::<NonClone>
        .validate_some(validate_non_zero_owned)
        .expect("missing optional value bypasses validation");
    assert_eq!(absent, None);
}
