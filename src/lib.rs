/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Qubit Argument
//!
//! Provides argument and state validation helpers for Rust applications.
//!
//! # Author
//!
//! Haixing Hu

pub mod lang;

/// Argument validation traits, functions, and error types.
pub mod argument {
    pub use crate::lang::argument::*;
}

pub use lang::argument::{
    ArgumentError,
    ArgumentResult,
    CollectionArgument,
    NumericArgument,
    OptionArgument,
    StringArgument,
    check_argument,
    check_argument_fmt,
    check_argument_with_message,
    check_bounds,
    check_element_index,
    check_position_index,
    check_position_indexes,
    check_state,
    check_state_with_message,
    require_element_non_null,
    require_equal,
    require_not_equal,
    require_null_or,
};
