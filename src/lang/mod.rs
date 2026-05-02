/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Argument Language Layer
//!
//! Provides argument and state validation helpers.
//!
//! # Author
//!
//! Haixing Hu

pub mod argument;

pub use argument::{
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
