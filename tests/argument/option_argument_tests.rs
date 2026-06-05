/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use qubit_argument::{
    ArgumentError,
    ArgumentResult,
    OptionArgument,
    require_null_or,
};

fn is_positive(value: &i32) -> bool {
    *value > 0
}

fn validate_positive(value: &i32) -> ArgumentResult<i32> {
    if *value > 0 {
        Ok(*value)
    } else {
        Err(ArgumentError::new("must be positive"))
    }
}

#[test]
fn test_named_function_instantiations_cover_all_option_branches() {
    assert_eq!(Some(1).require_non_null("value").unwrap(), 1);
    let error = None::<i32>.require_non_null("value").unwrap_err();
    assert_eq!(error.to_string(), "Parameter 'value' cannot be null");

    assert_eq!(
        Some(1)
            .require_non_null_and("value", is_positive, "must be positive")
            .unwrap(),
        1
    );
    let error = Some(0)
        .require_non_null_and("value", is_positive, "must be positive")
        .unwrap_err();
    assert_eq!(error.to_string(), "Parameter 'value' must be positive");
    let error = None::<i32>
        .require_non_null_and("value", is_positive, "must be positive")
        .unwrap_err();
    assert_eq!(error.to_string(), "Parameter 'value' cannot be null");

    assert_eq!(
        Some(2).validate_if_present("value", validate_positive).unwrap(),
        Some(2)
    );
    let error = Some(0).validate_if_present("value", validate_positive).unwrap_err();
    assert_eq!(error.to_string(), "must be positive");
    assert_eq!(
        None::<i32>.validate_if_present("value", validate_positive).unwrap(),
        None
    );

    assert_eq!(
        require_null_or("value", Some(3), is_positive, "must be positive").unwrap(),
        Some(3)
    );
    let error = require_null_or("value", Some(0), is_positive, "must be positive").unwrap_err();
    assert_eq!(error.to_string(), "Parameter 'value' must be positive");
    assert_eq!(
        require_null_or("value", None::<i32>, is_positive, "must be positive").unwrap(),
        None
    );
}

#[test]
fn require_non_null_and_and_validate_if_present() {
    let v = Some(10);
    assert_eq!(v.require_non_null("v").unwrap(), 10);

    let v = Some(18);
    let ok = v.require_non_null_and("age", |&a| a >= 18, "Must be at least 18 years old");
    assert!(ok.is_ok());
    assert_eq!(ok.unwrap(), 18);

    let v = Some(16);
    let err = v
        .require_non_null_and("age", |&a| a >= 18, "Must be at least 18 years old")
        .unwrap_err();
    assert!(err.to_string().contains("Must be at least 18 years old"));

    let none: Option<i32> = None;
    let err_none = none.require_non_null("x");
    assert!(err_none.is_err());
    let err = err_none.unwrap_err();
    assert!(err.to_string().contains("x"));
    assert!(err.to_string().contains("null"));

    // validate_if_present
    let port = Some(8080u16);
    let res = port.validate_if_present("port", |p| {
        if *p >= 1024 {
            Ok(*p)
        } else {
            Err("Port must be greater than or equal to 1024".into())
        }
    });
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), Some(8080));

    let bad = Some(80u16);
    let err2 = bad
        .validate_if_present("port", |p| {
            if *p >= 1024 {
                Ok(*p)
            } else {
                Err("Port must be greater than or equal to 1024".into())
            }
        })
        .unwrap_err();
    assert!(err2.to_string().contains("Port must be greater than or equal to 1024"));

    let none2: Option<u16> = None;
    let res2: ArgumentResult<Option<u16>> = none2.validate_if_present("port", |p| Ok(*p));
    assert!(res2.is_ok());
    assert_eq!(res2.unwrap(), None);
}

#[test]
fn require_null_or_free_function() {
    let some = Some(10);
    let ok = require_null_or("v", some, |&v| v > 0, "Must be positive");
    assert!(ok.is_ok());
    let val = ok.unwrap();
    assert_eq!(val, Some(10));
    assert_eq!(val.unwrap(), 10);

    let none: Option<i32> = None;
    let result = require_null_or("v", none, |&v| v > 0, "Must be positive");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);

    let bad = Some(-1);
    let err = require_null_or("v", bad, |&v| v > 0, "Must be positive");
    assert!(err.is_err());
    let err_msg = err.unwrap_err();
    assert!(err_msg.to_string().contains("Must be positive"));
}

#[test]
fn test_require_non_null_various_types() {
    // Test different types of Option
    let str_opt: Option<&str> = Some("hello");
    assert_eq!(str_opt.require_non_null("str").unwrap(), "hello");

    let string_opt: Option<String> = Some("world".to_string());
    assert_eq!(string_opt.require_non_null("string").unwrap(), "world");

    let bool_opt: Option<bool> = Some(true);
    assert!(bool_opt.require_non_null("bool").unwrap());

    let float_opt: Option<f64> = Some(2.71);
    assert_eq!(float_opt.require_non_null("float").unwrap(), 2.71);
}

#[test]
fn test_require_non_null_and_edge_cases() {
    // Test boundary values
    let zero: Option<i32> = Some(0);
    assert!(
        zero.require_non_null_and("zero", |&v| v >= 0, "Must be non-negative")
            .is_ok()
    );
    assert!(
        zero.require_non_null_and("zero", |&v| v > 0, "Must be positive")
            .is_err()
    );

    // Test maximum and minimum values
    let max: Option<i32> = Some(i32::MAX);
    assert!(max.require_non_null_and("max", |&v| v > 0, "Must be positive").is_ok());

    let min: Option<i32> = Some(i32::MIN);
    assert!(min.require_non_null_and("min", |&v| v < 0, "Must be negative").is_ok());
}

#[test]
fn test_validate_if_present_none() {
    // Test None case
    let none: Option<i32> = None;
    let result = none.validate_if_present("none", |_| Err("Should not be called".into()));
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_validate_if_present_with_complex_validation() {
    // Test complex validation logic
    let value: Option<String> = Some("test@example.com".to_string());
    let result = value.validate_if_present("email", |email| {
        if email.contains('@') && email.contains('.') {
            Ok(email.clone())
        } else {
            Err("Invalid email address".into())
        }
    });
    assert!(result.is_ok());

    let bad_email: Option<String> = Some("invalid".to_string());
    let result = bad_email.validate_if_present("email", |email| {
        if email.contains('@') && email.contains('.') {
            Ok(email.clone())
        } else {
            Err("Invalid email address".into())
        }
    });
    assert!(result.is_err());
}

#[test]
fn test_require_null_or_with_different_predicates() {
    // Test various predicates
    let value: Option<i32> = Some(5);

    // Range check
    assert!(require_null_or("value", value, |&v| (1..=10).contains(&v), "Must be between 1-10").is_ok());

    // Even number check
    assert!(require_null_or("value", Some(4), |&v| v % 2 == 0, "Must be even").is_ok());
    assert!(require_null_or("value", Some(3), |&v| v % 2 == 0, "Must be even").is_err());
}

#[test]
fn test_chaining_option_validation() {
    // Test chaining calls
    let value: Option<i32> = Some(10);
    let result = value.require_non_null("value").and_then(|v| {
        if v > 5 {
            Ok(v)
        } else {
            Err("Must be greater than 5".into())
        }
    });
    assert!(result.is_ok());
}

#[test]
fn test_require_non_null_and_success_branch() {
    // Ensure success branch is covered
    let value: Option<i32> = Some(20);
    let result = value.require_non_null_and("age", |&v| v >= 18, "Must be at least 18 years old");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 20);

    let value2: Option<String> = Some("hello".to_string());
    let result2 = value2.require_non_null_and("text", |s| !s.is_empty(), "Cannot be empty");
    assert!(result2.is_ok());
}

#[test]
fn test_validate_if_present_success_with_value() {
    // Test successful validation of Some value
    let value: Option<i32> = Some(100);
    let result = value.validate_if_present("value", |v| Ok(*v));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(100));

    // Test more complex validation
    let value2: Option<String> = Some("test".to_string());
    let result2 = value2.validate_if_present("text", |s| Ok(s.clone()));
    assert!(result2.is_ok());
}

#[test]
fn test_validate_if_present_uses_validator_output_value() {
    let value: Option<i32> = Some(10);
    let result = value.validate_if_present("value", |v| Ok(*v + 5));
    assert_eq!(result.unwrap(), Some(15));

    let text: Option<String> = Some("  hello  ".to_string());
    let result2 = text.validate_if_present("text", |s| Ok(s.trim().to_uppercase()));
    assert_eq!(result2.unwrap(), Some("HELLO".to_string()));
}

#[test]
fn test_require_null_or_success_with_value() {
    // Test Some value that satisfies the condition
    let value: Option<i32> = Some(50);
    let result = require_null_or("value", value, |&v| v > 0, "Must be positive");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(50));

    // Test boundary values
    let value2: Option<i32> = Some(0);
    let result2 = require_null_or("value", value2, |&v| v >= 0, "Must be non-negative");
    assert!(result2.is_ok());
    let val2 = result2.unwrap();
    assert_eq!(val2, Some(0));
    assert_eq!(val2.unwrap(), 0);
}

#[test]
fn test_all_none_cases() {
    // Ensure all None branches are covered
    let none: Option<i32> = None;
    assert!(none.require_non_null("test").is_err());

    let none2: Option<i32> = None;
    assert!(none2.validate_if_present("test", |v| Ok(*v)).is_ok());

    let none3: Option<i32> = None;
    assert!(require_null_or("test", none3, |&v| v > 0, "error").is_ok());
}

#[test]
fn test_validate_if_present_validator_return() {
    // Ensure validator return value is handled correctly
    use std::sync::Arc;
    use std::sync::atomic::{
        AtomicBool,
        Ordering,
    };

    // Test that validator is actually called
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    let value: Option<i32> = Some(42);
    let result = value.validate_if_present("value", move |v| {
        called_clone.store(true, Ordering::SeqCst);
        Ok(*v)
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(42));
    assert!(called.load(Ordering::SeqCst));

    // Test validator returning error
    let value2: Option<i32> = Some(5);
    let result2 = value2.validate_if_present("value", |v| if *v < 10 { Err("too small".into()) } else { Ok(*v) });
    assert!(result2.is_err());
}

#[test]
fn test_all_error_message_formats() {
    // Test all error message formats
    let none: Option<i32> = None;
    let err = none.require_non_null("test_param").unwrap_err();
    assert!(err.to_string().contains("test_param"));
    assert!(err.to_string().contains("null"));

    let value: Option<i32> = Some(5);
    let err2 = value
        .require_non_null_and("age", |&v| v >= 18, "Must be at least 18 years old")
        .unwrap_err();
    assert!(err2.to_string().contains("age"));
    assert!(err2.to_string().contains("Must be at least 18 years old"));

    let value3: Option<i32> = Some(-5);
    let err3 = require_null_or("number", value3, |&v| v > 0, "Must be positive").unwrap_err();
    assert!(err3.to_string().contains("number"));
    assert!(err3.to_string().contains("Must be positive"));
}

#[test]
fn test_require_non_null_and_with_none() {
    // Test case where require_non_null returns error in require_non_null_and
    let none: Option<i32> = None;
    let result = none.require_non_null_and("param", |&v| v > 0, "Must be positive");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("param"));
    assert!(err.to_string().contains("null"));
}

#[test]
fn test_require_non_null_and_ok_value() {
    // Ensure Ok(value) branch of require_non_null_and is covered
    let some: Option<i32> = Some(100);
    let result = some.require_non_null_and("value", |&v| v > 0, "Must be positive");
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val, 100);

    // Test different types
    let some_str: Option<String> = Some("test".to_string());
    let result2 = some_str.require_non_null_and("text", |s| !s.is_empty(), "Cannot be empty");
    assert!(result2.is_ok());
    let val2 = result2.unwrap();
    assert_eq!(val2, "test");

    // Test boundary values
    let some_zero: Option<i32> = Some(0);
    let result3 = some_zero.require_non_null_and("zero", |&v| v >= 0, "Must be non-negative");
    assert!(result3.is_ok());
    let val3 = result3.unwrap();
    assert_eq!(val3, 0);
}

#[test]
fn test_require_non_null_ok_branch() {
    // Ensure return value of success branch of require_non_null is used
    let some: Option<i32> = Some(42);
    let result = some.require_non_null("value");
    assert!(result.is_ok());
    let val = result.unwrap();
    assert_eq!(val, 42);

    // Test different types
    let some_str: Option<&str> = Some("hello");
    let result2 = some_str.require_non_null("text");
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "hello");

    let some_vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    let result3 = some_vec.require_non_null("vec");
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_require_non_null_error_branch() {
    // Test ArgumentError::new(format!(...)) branch of require_non_null
    let none: Option<i32> = None;
    let result = none.require_non_null("my_param");

    // Ensure error is returned
    assert!(result.is_err());

    // Get error and verify message content
    let error = result.unwrap_err();
    let error_msg = error.to_string();

    // Verify error message contains parameter name and "null"
    assert!(
        error_msg.contains("my_param"),
        "Error message should contain parameter name"
    );
    assert!(error_msg.contains("null"), "Error message should contain 'null'");
    assert!(error_msg.contains("cannot"), "Error message should contain 'cannot'");
    assert_eq!(error_msg, "Parameter 'my_param' cannot be null");

    // Test different parameter names
    let none2: Option<String> = None;
    let result2 = none2.require_non_null("another_param");
    assert!(result2.is_err());
    let error2 = result2.unwrap_err();
    assert!(error2.to_string().contains("another_param"));
}

#[test]
fn test_require_non_null_and_error_propagation() {
    // Test error propagation of let value = self.require_non_null(name)? in require_non_null_and
    let none: Option<i32> = None;

    // When require_non_null returns error, ? operator should propagate this error
    let result = none.require_non_null_and("param_name", |&v| v > 0, "Must be positive");

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = error.to_string();

    // Should be error from require_non_null, not from predicate
    assert!(error_msg.contains("param_name"));
    assert!(error_msg.contains("null"));

    // Test different types
    let none_str: Option<String> = None;
    let result2 = none_str.require_non_null_and("text_param", |s| !s.is_empty(), "Cannot be empty");
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("text_param"));
}

#[test]
fn test_validate_if_present_validator_error() {
    // Test error propagation of validator(value)? in validate_if_present
    let some: Option<i32> = Some(5);

    // When validator returns error, ? operator should propagate this error
    let result = some.validate_if_present("value", |v| {
        if *v < 10 {
            Err(ArgumentError::new("Value too small".to_string()))
        } else {
            Ok(*v)
        }
    });

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Value too small"));

    // Test different error messages
    let some2: Option<String> = Some("test".to_string());
    let result2 = some2.validate_if_present("text", |s| {
        if s.len() < 10 {
            Err(ArgumentError::new("String too short".to_string()))
        } else {
            Ok(s.clone())
        }
    });

    assert!(result2.is_err());
    let error2 = result2.unwrap_err();
    assert!(error2.to_string().contains("String too short"));
}

#[test]
fn test_require_non_null_closure_execution() {
    // Test to ensure the closure in ok_or_else is fully executed
    // This helps cover the region in line 158
    let none: Option<i32> = None;
    let result = none.require_non_null("test_parameter");

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = error.to_string();

    // Verify all parts of the format string are executed
    assert!(error_msg.contains("Parameter"));
    assert!(error_msg.contains("test_parameter"));
    assert!(error_msg.contains("cannot be null"));

    // Test with different parameter names to ensure format is always executed
    let none2: Option<String> = None;
    let result2 = none2.require_non_null("another_param_name");
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("another_param_name"));

    let none3: Option<f64> = None;
    let result3 = none3.require_non_null("x");
    assert!(result3.is_err());
    assert!(result3.unwrap_err().to_string().contains("x"));
}

#[test]
fn test_require_non_null_and_none_error_propagation() {
    // Test error propagation when require_non_null returns error in require_non_null_and
    // This ensures the ? operator and error path in line 165 are covered
    let none: Option<i32> = None;

    // This should trigger the error in require_non_null and propagate it
    let result = none.require_non_null_and("value", |&v| v > 0, "must be positive");

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = error.to_string();

    // The error should be from require_non_null, not from the predicate
    assert!(error_msg.contains("cannot be null"));
    assert!(!error_msg.contains("must be positive"));

    // Test with different types
    let none_str: Option<String> = None;
    let result2 = none_str.require_non_null_and("text", |s| !s.is_empty(), "cannot be empty");
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("cannot be null"));

    let none_vec: Option<Vec<i32>> = None;
    let result3 = none_vec.require_non_null_and("items", |v| !v.is_empty(), "must have items");
    assert!(result3.is_err());
    assert!(result3.unwrap_err().to_string().contains("cannot be null"));
}

#[test]
fn test_validate_if_present_error_propagation() {
    // Test error propagation when validator returns error
    // This ensures the ? operator in line 179 and error path are fully covered
    let some: Option<i32> = Some(5);

    // Validator that returns an error
    let result = some.validate_if_present("number", |v| {
        if *v < 10 {
            Err(ArgumentError::new("Value is too small".to_string()))
        } else {
            Ok(*v)
        }
    });

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Value is too small"));

    // Test with different error messages
    let some2: Option<String> = Some("ab".to_string());
    let result2 = some2.validate_if_present("text", |s| {
        if s.len() < 5 {
            Err(ArgumentError::new("String length must be at least 5".to_string()))
        } else {
            Ok(s.clone())
        }
    });

    assert!(result2.is_err());
    assert!(
        result2
            .unwrap_err()
            .to_string()
            .contains("String length must be at least 5")
    );

    // Test with custom error type
    let some3: Option<u32> = Some(0);
    let result3 = some3.validate_if_present("count", |&n| {
        if n == 0 {
            Err(ArgumentError::new("Count cannot be zero".to_string()))
        } else {
            Ok(n)
        }
    });

    assert!(result3.is_err());
    assert!(result3.unwrap_err().to_string().contains("Count cannot be zero"));
}

#[test]
fn test_require_non_null_and_predicate_false_branch() {
    // Ensure the false branch of predicate check is covered
    let some: Option<i32> = Some(5);

    // Predicate returns false, should hit line 167
    let result = some.require_non_null_and("age", |&v| v >= 18, "must be at least 18");

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = error.to_string();

    assert!(error_msg.contains("age"));
    assert!(error_msg.contains("must be at least 18"));

    // Test with different predicates
    let some2: Option<String> = Some("".to_string());
    let result2 = some2.require_non_null_and("name", |s| !s.is_empty(), "cannot be empty");
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("cannot be empty"));
}

#[test]
fn test_require_non_null_and_predicate_true_branch() {
    // Ensure the true branch of predicate check is covered (line 169)
    let some: Option<i32> = Some(25);

    // Predicate returns true, should return Ok(value)
    let result = some.require_non_null_and("age", |&v| v >= 18, "must be at least 18");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 25);

    // Test with different valid values
    let some2: Option<String> = Some("hello".to_string());
    let result2 = some2.require_non_null_and("text", |s| !s.is_empty(), "cannot be empty");
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "hello");

    let some3: Option<f64> = Some(2.71);
    let result3 = some3.require_non_null_and("pi", |&v| v > 0.0, "must be positive");
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), 2.71);
}

#[test]
fn test_validate_if_present_success_path() {
    // Ensure Ok(self) path in line 180 is covered
    let some: Option<i32> = Some(100);

    // Validator succeeds
    let result = some.validate_if_present("value", |v| {
        if *v >= 0 {
            Ok(*v)
        } else {
            Err(ArgumentError::new("must be non-negative".to_string()))
        }
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(100));

    // Test with different validators
    let some2: Option<String> = Some("test@example.com".to_string());
    let result2 = some2.validate_if_present("email", |s| {
        if s.contains('@') {
            Ok(s.clone())
        } else {
            Err(ArgumentError::new("invalid email".to_string()))
        }
    });

    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), Some("test@example.com".to_string()));
}

#[test]
fn test_require_null_or_none_branch() {
    // Ensure None branch in line 229 is covered
    let none: Option<i32> = None;
    let result = require_null_or("value", none, |&v| v > 0, "must be positive");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);

    // Test with different types
    let none2: Option<String> = None;
    let result2 = require_null_or("text", none2, |s: &String| !s.is_empty(), "cannot be empty");
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), None);
}

#[test]
fn test_require_null_or_some_success_branch() {
    // Ensure Some branch with predicate true (line 234) is covered
    let some: Option<i32> = Some(10);
    let result = require_null_or("value", some, |&v| v > 0, "must be positive");

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(10));

    // Test with different valid values
    let some2: Option<String> = Some("hello".to_string());
    let result2 = require_null_or("text", some2, |s| !s.is_empty(), "cannot be empty");
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), Some("hello".to_string()));
}

#[test]
fn test_require_null_or_some_failure_branch() {
    // Ensure Some branch with predicate false (line 232) is covered
    let some: Option<i32> = Some(-5);
    let result = require_null_or("value", some, |&v| v > 0, "must be positive");

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = error.to_string();

    assert!(error_msg.contains("value"));
    assert!(error_msg.contains("must be positive"));

    // Test with different failing predicates
    let some2: Option<String> = Some("".to_string());
    let result2 = require_null_or("text", some2, |s: &String| !s.is_empty(), "cannot be empty");
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("cannot be empty"));
}

#[test]
fn test_comprehensive_type_coverage() {
    // Test with various types to ensure all code paths work with different types

    // Test with Vec
    let vec_opt: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    assert!(vec_opt.require_non_null("vec").is_ok());

    // Test with tuple
    let tuple_opt: Option<(i32, String)> = Some((1, "test".to_string()));
    assert!(tuple_opt.require_non_null("tuple").is_ok());

    // Test with Option<Option<T>>
    let nested: Option<Option<i32>> = Some(Some(42));
    assert!(nested.require_non_null("nested").is_ok());

    // Test with Result type
    let result_opt: Option<Result<i32, String>> = Some(Ok(10));
    assert!(result_opt.require_non_null("result").is_ok());
}

#[test]
fn test_multiple_error_scenarios() {
    // Test to ensure all error creation paths are covered

    // Error from require_non_null with various parameter names
    let test_cases = vec![
        ("a", None::<i32>),
        ("param", None::<i32>),
        ("very_long_parameter_name", None::<i32>),
        ("x1", None::<i32>),
    ];

    for (name, value) in test_cases {
        let result = value.require_non_null(name);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains(name));
    }

    // Error from require_non_null_and predicate
    let some_val = Some(5);
    let result = some_val.require_non_null_and("val", |&v| v > 10, "must be greater than 10");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must be greater than 10"));

    // Error from validate_if_present validator
    let some_val2 = Some(3);
    let result2 = some_val2.validate_if_present("num", |&n| {
        if n < 5 {
            Err(ArgumentError::new("too small".to_string()))
        } else {
            Ok(n)
        }
    });
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("too small"));

    // Error from require_null_or
    let some_val3 = Some(0);
    let result3 = require_null_or("zero", some_val3, |&v| v != 0, "cannot be zero");
    assert!(result3.is_err());
    assert!(result3.unwrap_err().to_string().contains("cannot be zero"));
}

#[test]
fn test_require_non_null_with_explicit_error_check() {
    // Test multiple None values with different names
    for name in &["a", "b", "c", "param1", "param2", "value", "test", "x", "y", "z"] {
        let none: Option<i32> = None;
        match none.require_non_null(name) {
            Ok(_) => panic!("Expected error"),
            Err(err) => {
                let msg = err.to_string();
                assert!(
                    msg.contains(name),
                    "Error message should contain parameter name '{}'",
                    name
                );
                assert!(
                    msg.contains("cannot be null"),
                    "Error message should contain 'cannot be null'"
                );
                // Force error message evaluation
                let _ = format!("{:?}", err);
                let _ = err.message();
            }
        }
    }
}

#[test]
fn test_require_non_null_and_with_explicit_branches() {
    // Test to ensure all branches in require_non_null_and are covered

    // Branch 1: None input -> error from require_non_null
    for i in 0..5 {
        let none: Option<i32> = None;
        let name = format!("param_{}", i);
        match none.require_non_null_and(&name, |&v| v > 0, "positive") {
            Ok(_) => panic!("Expected error"),
            Err(err) => {
                assert!(err.to_string().contains(&name));
                assert!(err.to_string().contains("null"));
            }
        }
    }

    // Branch 2: Some input, predicate fails
    for i in 0..5 {
        let some: Option<i32> = Some(i);
        let name = format!("val_{}", i);
        match some.require_non_null_and(&name, |&v| v > 10, "must be > 10") {
            Ok(_) => panic!("Expected error"),
            Err(err) => {
                assert!(err.to_string().contains(&name));
                assert!(err.to_string().contains("must be > 10"));
            }
        }
    }

    // Branch 3: Some input, predicate succeeds
    for i in 11..16 {
        let some: Option<i32> = Some(i);
        let name = format!("val_{}", i);
        match some.require_non_null_and(&name, |&v| v > 10, "must be > 10") {
            Ok(val) => assert_eq!(val, i),
            Err(_) => panic!("Expected success"),
        }
    }
}

#[test]
fn test_validate_if_present_with_explicit_branches() {
    // Test to ensure all branches in validate_if_present are covered

    // Branch 1: None input
    for _i in 0..5 {
        let none: Option<i32> = None;
        match none.validate_if_present("test", |v| Ok(*v)) {
            Ok(result) => assert_eq!(result, None),
            Err(_) => panic!("Expected None"),
        }
    }

    // Branch 2: Some input, validator returns Ok
    for i in 0..5 {
        let some: Option<i32> = Some(i);
        let name = format!("val_{}", i);
        match some.validate_if_present(&name, |v| Ok(*v)) {
            Ok(result) => assert_eq!(result, Some(i)),
            Err(_) => panic!("Expected success"),
        }
    }

    // Branch 3: Some input, validator returns Err
    for i in 0..5 {
        let some: Option<i32> = Some(i);
        let name = format!("val_{}", i);
        let error_msg = format!("error_{}", i);
        match some.validate_if_present(&name, |_v| Err(ArgumentError::new(error_msg.clone()))) {
            Ok(_) => panic!("Expected error"),
            Err(err) => {
                assert!(err.to_string().contains(&error_msg));
            }
        }
    }
}

#[test]
fn test_require_null_or_with_explicit_branches() {
    // Test all branches of require_null_or explicitly

    // Branch 1: None input
    for i in 0..5 {
        let name = format!("param_{}", i);
        let none: Option<i32> = None;
        match require_null_or(&name, none, |&v| v > 0, "positive") {
            Ok(result) => assert_eq!(result, None),
            Err(_) => panic!("Expected None"),
        }
    }

    // Branch 2: Some input, predicate succeeds
    for i in 1..6 {
        let name = format!("val_{}", i);
        let some: Option<i32> = Some(i);
        match require_null_or(&name, some, |&v| v > 0, "positive") {
            Ok(result) => assert_eq!(result, Some(i)),
            Err(_) => panic!("Expected success"),
        }
    }

    // Branch 3: Some input, predicate fails
    for i in -5..0 {
        let name = format!("val_{}", i);
        let some: Option<i32> = Some(i);
        match require_null_or(&name, some, |&v| v > 0, "positive") {
            Ok(_) => panic!("Expected error"),
            Err(err) => {
                assert!(err.to_string().contains(&name));
                assert!(err.to_string().contains("positive"));
            }
        }
    }
}

#[test]
fn test_all_functions_with_various_types_and_names() {
    // Test with many different types to ensure generic implementations are covered

    // Test require_non_null with different types
    Some(42i32).require_non_null("i32").unwrap();
    Some(42i64).require_non_null("i64").unwrap();
    Some(42u32).require_non_null("u32").unwrap();
    Some(42u64).require_non_null("u64").unwrap();
    Some(2.71f32).require_non_null("f32").unwrap();
    Some(2.71f64).require_non_null("f64").unwrap();
    Some("str").require_non_null("str").unwrap();
    Some(String::from("string")).require_non_null("string").unwrap();
    Some(true).require_non_null("bool").unwrap();
    Some('c').require_non_null("char").unwrap();
    Some(vec![1, 2, 3]).require_non_null("vec").unwrap();
    Some((1, 2)).require_non_null("tuple").unwrap();

    // Test with different parameter names to trigger format! in different ways
    let test_names = vec![
        "x",
        "y",
        "z",
        "a",
        "b",
        "c",
        "value",
        "param",
        "arg",
        "input",
        "data",
        "my_value",
        "some_param",
        "the_arg",
        "very_long_parameter_name_for_testing",
        "param_1",
        "param_2",
        "param_3",
    ];

    for name in test_names {
        let none: Option<i32> = None;
        let _ = none.require_non_null(name);

        let some = Some(42);
        let _ = some.require_non_null(name);
    }
}
