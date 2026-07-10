// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for ownership-preserving `Option` argument validation.

use std::cell::Cell;

use qubit_argument::{ArgumentError, ArgumentErrorKind, OptionArgument};

/// A value that cannot be cloned, used to verify ownership-preserving APIs.
#[derive(Debug, PartialEq, Eq)]
struct NonClone(u32);

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
                Err(ArgumentError::structured(
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
            Err(ArgumentError::structured(
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
