// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Argument Validation
//!
//! Provides argument validation functionality similar to Java's `Argument`
//! class, but with a design more suitable for Rust conventions.
//!
//! # Module Organization
//!
//! - `argument_error`: Error type definitions
//! - `argument_path`: Structured argument paths
//! - `argument_value`: Lossless primitive numeric values
//! - `numeric_argument`: Numeric argument validation
//! - `string_argument`: String argument validation
//! - `collection_argument`: Collection argument validation
//! - `constraint`: Reusable constraint vocabulary
//! - `option_argument`: Option argument validation
//! - `condition`: Condition and state validation
//!
//! # Design Philosophy
//!
//! This module uses Rust's trait extension pattern to provide validation
//! methods for various types. Compared to Java's static methods, this approach
//! is more idiomatic in Rust and supports method chaining.
//!
//! # Usage Examples
//!
//! ```rust
//! use qubit_argument::argument::{
//!     NumericArgument, StringArgument, CollectionArgument, ArgumentResult
//! };
//! use regex::Regex;
//!
//! fn process_user_input(
//!     age: i32,
//!     username: &str,
//!     tags: &[String],
//! ) -> ArgumentResult<()> {
//!     // Numeric validation
//!     let age = age.require_in_closed_range("age", 0, 150)?;
//!
//!     // String validation (regex + chaining)
//!     let username_pattern = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{2,19}$").unwrap();
//!     let username = username
//!         .require_non_blank("username")?
//!         .require_match("username", &username_pattern)?;
//!
//!     // Collection validation (chaining)
//!     let tags = tags
//!         .require_non_empty("tags")?
//!         .require_length_at_most("tags", 10)?;
//!
//!     println!("Age: {}, Username: {}, Tag count: {}", age, username, tags.len());
//!     Ok(())
//! }
//! ```

pub mod argument_error;
mod argument_path;
mod argument_value;
pub mod collection_argument;
pub mod condition;
mod constraint;
pub mod numeric_argument;
pub mod option_argument;
pub mod string_argument;

// Re-export main types and traits
pub use argument_error::{
    ArgumentError,
    ArgumentErrorKind,
    ArgumentResult,
};
pub use argument_path::ArgumentPath;
pub use argument_value::ArgumentValue;
pub use collection_argument::{
    CollectionArgument,
    require_element_non_null,
};
pub use condition::{
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
pub use constraint::{
    ArgumentBound,
    ComparisonConstraint,
    IndexRole,
    LengthConstraint,
    PatternExpectation,
    RangeConstraint,
};
pub use numeric_argument::{
    NumericArgument,
    require_equal,
    require_not_equal,
};
pub use option_argument::{
    OptionArgument,
    require_null_or,
};
pub use string_argument::StringArgument;
