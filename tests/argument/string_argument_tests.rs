/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use qubit_argument::StringArgument;
use regex::Regex;

#[test]
fn non_blank() {
    let result = "hello".require_non_blank("s");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");

    assert!("   ".require_non_blank("s").is_err());

    let s = String::from("world");
    let result2 = s.require_non_blank("s");
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "world");

    // Test error cases
    let err = "".require_non_blank("empty");
    assert!(err.is_err());
    let err_msg = err.unwrap_err();
    assert!(err_msg.to_string().contains("empty"));

    let err2 = "  \t  ".require_non_blank("whitespace");
    assert!(err2.is_err());

    let empty_string = String::from("");
    assert!(empty_string.require_non_blank("s").is_err());
}

#[test]
fn length_checks_and_range() {
    let r1 = "abc".require_length_be("s", 3);
    assert!(r1.is_ok());
    assert_eq!(r1.unwrap(), "abc");

    // require_length_be error cases
    let err1 = "abc".require_length_be("s", 2);
    assert!(err1.is_err());
    let err_msg1 = err1.unwrap_err();
    assert!(err_msg1.to_string().contains("s"));

    let s = String::from("test");
    let err1b = s.require_length_be("s", 5);
    assert!(err1b.is_err());

    let r2 = "abc".require_length_at_least("s", 3);
    assert!(r2.is_ok());
    assert_eq!(r2.unwrap(), "abc");

    // require_length_at_least error cases
    let err2 = "ab".require_length_at_least("s", 3);
    assert!(err2.is_err());
    let err_msg2 = err2.unwrap_err();
    assert!(err_msg2.to_string().contains("at least"));

    let s2 = String::from("x");
    let err2b = s2.require_length_at_least("s", 5);
    assert!(err2b.is_err());

    let r3 = "abc".require_length_at_most("s", 3);
    assert!(r3.is_ok());
    assert_eq!(r3.unwrap(), "abc");

    // require_length_at_most error cases
    let err3 = "abcd".require_length_at_most("s", 3);
    assert!(err3.is_err());
    let err_msg3 = err3.unwrap_err();
    assert!(err_msg3.to_string().contains("at most"));

    let s3 = String::from("toolong");
    let err3b = s3.require_length_at_most("s", 3);
    assert!(err3b.is_err());

    let r4 = "abcd".require_length_in_range("s", 3, 5);
    assert!(r4.is_ok());
    assert_eq!(r4.unwrap(), "abcd");

    // require_length_in_range error cases - less than min
    let err4a = "ab".require_length_in_range("s", 3, 5);
    assert!(err4a.is_err());

    // require_length_in_range error cases - greater than max (this is important!)
    let err4b = "abcdef".require_length_in_range("s", 3, 5);
    assert!(err4b.is_err());
    let err_msg4b = err4b.unwrap_err();
    assert!(err_msg4b.to_string().contains("range"));

    let s4 = String::from("verylongstring");
    let err4c = s4.require_length_in_range("s", 1, 5);
    assert!(err4c.is_err());
}

#[test]
fn regex_match_and_not_match() {
    let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    let r1 = "user@example.com".require_match("email", &email_re);
    assert!(r1.is_ok());
    assert_eq!(r1.unwrap(), "user@example.com");

    // require_match error cases
    let err1 = "bad@com".require_match("email", &email_re);
    assert!(err1.is_err());
    let err_msg1 = err1.unwrap_err();
    assert!(err_msg1.to_string().contains("email"));
    assert!(err_msg1.to_string().contains("match"));

    let s = String::from("invalid");
    let err1b = s.require_match("email", &email_re);
    assert!(err1b.is_err());

    let digits = Regex::new(r"^\d+$").unwrap();
    let r2 = "abc".require_not_match("id", &digits);
    assert!(r2.is_ok());
    assert_eq!(r2.unwrap(), "abc");

    // require_not_match error cases
    let err2 = "123".require_not_match("id", &digits);
    assert!(err2.is_err());
    let err_msg2 = err2.unwrap_err();
    assert!(err_msg2.to_string().contains("id"));
    assert!(err_msg2.to_string().contains("cannot match"));

    let s2 = String::from("456");
    let err2b = s2.require_not_match("id", &digits);
    assert!(err2b.is_err());
}

#[test]
fn multibyte_strings_len_is_bytes() {
    // "汉" in UTF-8 is 3 bytes; emoji like "😀" is 4 bytes
    let han = "汉"; // 3 bytes
    assert!(han.require_length_be("c", 3).is_ok());
    assert!(han.require_length_be("c", 1).is_err());

    let emoji = "😀"; // 4 bytes
    assert!(emoji.require_length_be("e", 4).is_ok());
    assert!(emoji.require_length_in_range("e", 4, 4).is_ok());
}

#[test]
fn test_empty_string() {
    // Test empty string
    assert!("".require_non_blank("empty").is_err());
    assert!("".require_length_be("empty", 0).is_ok());
    assert!("".require_length_at_least("empty", 0).is_ok());
    assert!("".require_length_at_most("empty", 0).is_ok());
    assert!("".require_length_at_most("empty", 10).is_ok());
}

#[test]
fn test_whitespace_only() {
    // Test strings containing only whitespace
    assert!("   ".require_non_blank("spaces").is_err());
    assert!("\t".require_non_blank("tab").is_err());
    assert!("\n".require_non_blank("newline").is_err());
    assert!(" \t\n ".require_non_blank("mixed").is_err());
}

#[test]
fn test_string_type() {
    // Test String type
    let s = String::from("hello");
    assert!(s.require_non_blank("string").is_ok());
    assert!(s.require_length_be("string", 5).is_ok());
    assert!(s.require_length_at_least("string", 3).is_ok());
    assert!(s.require_length_at_most("string", 10).is_ok());
    assert!(s.require_length_in_range("string", 3, 10).is_ok());
}

#[test]
fn test_regex_edge_cases() {
    // Test regex edge cases
    let any = Regex::new(r".*").unwrap();
    assert!("anything".require_match("any", &any).is_ok());
    assert!("".require_match("empty", &any).is_ok());

    let none = Regex::new(r"^$").unwrap();
    assert!("".require_match("empty", &none).is_ok());
    assert!("x".require_match("not_empty", &none).is_err());

    // Test require_not_match
    assert!("abc".require_not_match("text", &none).is_ok());
    assert!("".require_not_match("empty", &none).is_err());
}

#[test]
fn test_length_boundary() {
    // Test length boundary cases
    let text = "12345";

    // Exactly equal
    assert!(text.require_length_be("text", 5).is_ok());
    assert!(text.require_length_at_least("text", 5).is_ok());
    assert!(text.require_length_at_most("text", 5).is_ok());
    assert!(text.require_length_in_range("text", 5, 5).is_ok());

    // Just less than
    assert!(text.require_length_be("text", 6).is_err());
    assert!(text.require_length_at_most("text", 4).is_err());

    // Just greater than
    assert!(text.require_length_at_least("text", 6).is_err());
}

#[test]
fn test_chaining() {
    // Test chaining calls
    let username = "alice";
    let result = username
        .require_non_blank("username")
        .and_then(|u| u.require_length_at_least("username", 3))
        .and_then(|u| u.require_length_at_most("username", 20));
    assert!(result.is_ok());

    // Test chaining call failure
    let short = "ab";
    let result = short
        .require_non_blank("username")
        .and_then(|u| u.require_length_at_least("username", 3));
    assert!(result.is_err());
}

#[test]
fn test_special_characters() {
    // Test special characters
    let special = "hello@world#123";
    assert!(special.require_non_blank("special").is_ok());
    assert!(special.require_length_be("special", 15).is_ok());

    let pattern = Regex::new(r"[a-z@#0-9]+").unwrap();
    assert!(special.require_match("special", &pattern).is_ok());
}

#[test]
fn test_very_long_string() {
    // Test very long string
    let long = "a".repeat(1000);
    assert!(long.require_non_blank("long").is_ok());
    assert!(long.require_length_be("long", 1000).is_ok());
    assert!(long.require_length_at_least("long", 500).is_ok());
    assert!(long.require_length_at_most("long", 2000).is_ok());
    assert!(long.require_length_in_range("long", 500, 2000).is_ok());
}

#[test]
fn test_all_success_branches_for_str() {
    // Ensure all success branches of str type are covered
    let text = "hello";

    // require_non_blank success
    assert!(text.require_non_blank("text").is_ok());
    assert_eq!(text.require_non_blank("text").unwrap(), "hello");

    // require_length_be success
    assert!(text.require_length_be("text", 5).is_ok());
    assert_eq!(text.require_length_be("text", 5).unwrap(), "hello");

    // require_length_at_least success
    assert!(text.require_length_at_least("text", 5).is_ok());
    assert_eq!(text.require_length_at_least("text", 5).unwrap(), "hello");
    assert!(text.require_length_at_least("text", 3).is_ok());

    // require_length_at_most success
    assert!(text.require_length_at_most("text", 5).is_ok());
    assert_eq!(text.require_length_at_most("text", 5).unwrap(), "hello");
    assert!(text.require_length_at_most("text", 10).is_ok());

    // require_length_in_range success
    assert!(text.require_length_in_range("text", 3, 7).is_ok());
    assert_eq!(text.require_length_in_range("text", 3, 7).unwrap(), "hello");
    assert!(text.require_length_in_range("text", 5, 5).is_ok());

    // require_match success
    use regex::Regex;
    let pattern = Regex::new(r"^[a-z]+$").unwrap();
    assert!(text.require_match("text", &pattern).is_ok());
    assert_eq!(text.require_match("text", &pattern).unwrap(), "hello");

    // require_not_match success
    let digit_pattern = Regex::new(r"\d").unwrap();
    assert!(text.require_not_match("text", &digit_pattern).is_ok());
    assert_eq!(text.require_not_match("text", &digit_pattern).unwrap(), "hello");
}

#[test]
fn test_all_success_branches_for_string() {
    // Ensure all success branches of String type are covered
    let text = String::from("world");

    // require_non_blank success
    assert!(text.require_non_blank("text").is_ok());
    assert_eq!(text.require_non_blank("text").unwrap(), "world");

    // require_length_be success
    assert!(text.require_length_be("text", 5).is_ok());
    assert_eq!(text.require_length_be("text", 5).unwrap(), "world");

    // require_length_at_least success
    assert!(text.require_length_at_least("text", 5).is_ok());
    assert_eq!(text.require_length_at_least("text", 5).unwrap(), "world");
    assert!(text.require_length_at_least("text", 3).is_ok());

    // require_length_at_most success
    assert!(text.require_length_at_most("text", 5).is_ok());
    assert_eq!(text.require_length_at_most("text", 5).unwrap(), "world");
    assert!(text.require_length_at_most("text", 10).is_ok());

    // require_length_in_range success
    assert!(text.require_length_in_range("text", 3, 7).is_ok());
    assert_eq!(text.require_length_in_range("text", 3, 7).unwrap(), "world");
    assert!(text.require_length_in_range("text", 5, 5).is_ok());

    // require_match success
    use regex::Regex;
    let pattern = Regex::new(r"^[a-z]+$").unwrap();
    assert!(text.require_match("text", &pattern).is_ok());
    assert_eq!(text.require_match("text", &pattern).unwrap(), "world");

    // require_not_match success
    let digit_pattern = Regex::new(r"\d").unwrap();
    assert!(text.require_not_match("text", &digit_pattern).is_ok());
    assert_eq!(text.require_not_match("text", &digit_pattern).unwrap(), "world");
}

#[test]
fn test_string_impl_returns_same_reference_after_validation() {
    let text = String::from("delegate");
    let validated = text.require_non_blank("text").unwrap();
    assert!(std::ptr::eq(validated, &text));

    let regex = Regex::new(r"^[a-z]+$").unwrap();
    let validated2 = text.require_match("text", &regex).unwrap();
    assert!(std::ptr::eq(validated2, &text));
}

#[test]
fn test_length_in_range_min_boundary() {
    // Test min_length boundary
    let text = "abc";
    assert!(text.require_length_in_range("text", 3, 10).is_ok());
    assert!(text.require_length_in_range("text", 4, 10).is_err());
}

#[test]
fn test_length_in_range_max_boundary() {
    // Test max_length boundary
    let text = "abcde";
    assert!(text.require_length_in_range("text", 1, 5).is_ok());
    assert!(text.require_length_in_range("text", 1, 4).is_err());

    // Test String type
    let text2 = String::from("abcdef");
    assert!(text2.require_length_in_range("text", 1, 5).is_err());
}

#[test]
fn test_length_in_range_both_boundaries() {
    // Test both min and max boundaries
    let text = "abc";

    // Less than min
    let result = text.require_length_in_range("text", 5, 10);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must be in"));

    // Greater than max
    let text2 = "abcdefghijk";
    let result2 = text2.require_length_in_range("text", 1, 5);
    assert!(result2.is_err());

    // String type less than min
    let text3 = String::from("ab");
    assert!(text3.require_length_in_range("text", 5, 10).is_err());

    // String type greater than max
    let text4 = String::from("abcdefghijk");
    assert!(text4.require_length_in_range("text", 1, 5).is_err());
}
