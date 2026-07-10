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
//! - `bounds`: Bounds and custom argument validation
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
//! use qubit_argument::{
//!     NumericArgument, StringArgument, CollectionArgument, ArgumentResult
//! };
//! #[cfg(feature = "regex")]
//! use regex::Regex;
//!
//! fn process_user_input(
//!     age: i32,
//!     username: &str,
//!     tags: &[String],
//! ) -> ArgumentResult<()> {
//!     // Numeric validation
//!     let age = age.require_in_range("age", 0..=150)?;
//!
//!     // String validation, with optional regex validation when enabled
//!     let username = username.require_non_blank("username")?;
//!     #[cfg(feature = "regex")]
//!     let username = {
//!         let username_pattern = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{2,19}$")
//!             .expect("username pattern is valid");
//!         username.require_match("username", &username_pattern)?
//!     };
//!
//!     // Collection validation (chaining)
//!     let tags = tags
//!         .require_non_empty("tags")?
//!         .require_len_at_most("tags", 10)?;
//!
//!     println!("Age: {}, Username: {}, Tag count: {}", age, username, tags.len());
//!     Ok(())
//! }
//! ```

mod argument_error;
mod argument_path;
mod argument_value;
mod bounds;
mod collection_argument;
mod constraint;
mod numeric_argument;
mod option_argument;
mod string_argument;

// Re-export main types and traits
pub use argument_error::{ArgumentError, ArgumentErrorKind, ArgumentResult};
pub use argument_path::ArgumentPath;
pub use argument_value::ArgumentValue;
pub use bounds::{
    check_bounds, check_element_index, check_position_index, check_position_range, require_that,
};
pub use collection_argument::CollectionArgument;
pub use constraint::{
    ArgumentBound, ComparisonConstraint, IndexRole, LengthConstraint, PatternExpectation,
    RangeConstraint,
};
pub use numeric_argument::NumericArgument;
pub use option_argument::OptionArgument;
pub use string_argument::StringArgument;
