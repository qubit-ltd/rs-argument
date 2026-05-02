# Argument Validation Module

Author: Haixing Hu

## Overview

This module provides argument validation functionality with a design that follows Rust idioms. Through trait extension patterns, it provides validation methods for various types and supports method chaining.

## Module Structure

```
argument/
├── argument_error.rs       ## Error type definitions
├── numeric_argument.rs     ## Numeric argument validation
├── string_argument.rs      ## String argument validation
├── collection_argument.rs  ## Collection argument validation
├── option_argument.rs      ## Option argument validation
├── condition.rs            ## Condition and state validation
└── mod.rs                  ## Module entry point
```

## Core Features

### 1. Numeric Validation (`NumericArgument`)

Supports the built-in numeric types implemented by this crate: signed integers
(`i8`, `i16`, `i32`, `i64`, `i128`, `isize`), unsigned integers (`u8`, `u16`,
`u32`, `u64`, `u128`, `usize`), and floats (`f32`, `f64`). Floating-point
validations reject `NaN`.

```rust
use qubit_argument::argument::NumericArgument;

// Validate non-negative
let count = 10;
let count = count.require_non_negative("count")?;

// Validate range
let volume = 50;
let volume = volume.require_in_closed_range("volume", 0, 100)?;

// Method chaining
let age = 25;
let age = age
    .require_non_negative("age")
    .and_then(|a| a.require_in_closed_range("age", 0, 150))?;
```

**Available methods:**
- `require_zero()` - Validate zero
- `require_non_zero()` - Validate non-zero
- `require_positive()` - Validate positive
- `require_non_negative()` - Validate non-negative
- `require_negative()` - Validate negative
- `require_non_positive()` - Validate non-positive
- `require_in_closed_range()` - Closed interval [min, max]
- `require_in_open_range()` - Open interval (min, max)
- `require_in_left_open_range()` - Left-open, right-closed (min, max]
- `require_in_right_open_range()` - Left-closed, right-open [min, max)
- `require_less()` - Less than
- `require_less_equal()` - Less than or equal
- `require_greater()` - Greater than
- `require_greater_equal()` - Greater than or equal

### 2. String Validation (`StringArgument`)

Supports `&str` and `String` types.

```rust
use qubit_argument::argument::StringArgument;

// Validate non-blank
let username = "alice";
let username = username.require_non_blank("username")?;

// Validate length
let password = "secret123";
let password = password
    .require_length_at_least("password", 8)
    .and_then(|p| p.require_length_at_most("password", 20))?;

// Regex matching
use regex::Regex;
let email = "user@example.com";
let pattern = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")?;
let email = email.require_match("email", &pattern)?;
```

**Available methods:**
- `require_non_blank()` - Validate non-blank
- `require_length_be()` - Length equals
- `require_length_at_least()` - Minimum length
- `require_length_at_most()` - Maximum length
- `require_length_in_range()` - Length range
- `require_match()` - Regex match
- `require_not_match()` - Regex not match

### 3. Collection Validation (`CollectionArgument`)

Supports `&[T]` and `Vec<T>` types.

```rust
use qubit_argument::argument::CollectionArgument;

// Validate non-empty
let items = vec![1, 2, 3];
let items = items.require_non_empty("items")?;

// Validate length
let tags = vec!["rust", "programming"];
let tags = tags
    .require_non_empty("tags")
    .and_then(|t| t.require_length_at_most("tags", 10))?;
```

**Available methods:**
- `require_non_empty()` - Validate non-empty
- `require_length_be()` - Length equals
- `require_length_at_least()` - Minimum length
- `require_length_at_most()` - Maximum length
- `require_length_in_range()` - Length range

### 4. Option Validation (`OptionArgument`)

Supports `Option<T>` types.

```rust
use qubit_argument::argument::OptionArgument;

// Validate non-null
let timeout: Option<u64> = Some(30);
let timeout = timeout.require_non_null("timeout")?;

// Validate non-null and condition
let port: Option<u16> = Some(8080);
let port = port.require_non_null_and(
    "port",
    |&p| p >= 1024,
    "Must be greater than or equal to 1024"
)?;

// Optional validation
let max_conn: Option<usize> = Some(100);
let max_conn = max_conn.validate_if_present("max_connections", |c| {
    if *c > 10000 {
        Err("Too many connections".into())
    } else {
        Ok(*c)
    }
})?;
```

**Available methods:**
- `require_non_null()` - Validate non-None
- `require_non_null_and()` - Validate non-None and condition
- `validate_if_present()` - Validate if present

### 5. Condition Validation

General condition and state validation functions.

```rust
use qubit_argument::argument::{check_argument, check_state, check_bounds};

// Basic condition check
let is_valid = true;
check_argument(is_valid)?;

// Check with message
check_argument_with_message(count > 0, "Count must be positive")?;

// State check
check_state(is_initialized)?;

// Bounds check
check_bounds(offset, length, total_length)?;

// Index check
let index = check_element_index(5, list_size)?;
```

**Available functions:**
- `check_argument()` - Check argument condition
- `check_argument_with_message()` - Argument check with message
- `check_argument_fmt()` - Argument check with formatted message
- `check_state()` - Check state condition
- `check_state_with_message()` - State check with message
- `check_bounds()` - Bounds check
- `check_element_index()` - Element index check
- `check_position_index()` - Position index check
- `check_position_indexes()` - Position index range check

## Error Handling

All validation methods return `ArgumentResult<T>`, which is a type alias for `Result<T, ArgumentError>`.

```rust
use qubit_argument::argument::{ArgumentError, ArgumentResult};

fn validate_config(port: u16, timeout: u64) -> ArgumentResult<()> {
    port.require_in_closed_range("port", 1024, 65535)?;
    timeout.require_positive("timeout")?;
    Ok(())
}

// Error handling
match validate_config(80, 30) {
    Ok(_) => println!("Configuration is valid"),
    Err(e) => eprintln!("Configuration error: {}", e.message()),
}
```

## Design Philosophy

1. **Type Safety**: Leverage Rust's type system for compile-time safety
2. **Zero-cost Abstractions**: Same performance as manual checks after compilation
3. **Method Chaining**: Return `Result<Self>` to support elegant chained validation
4. **Clear Errors**: Provide friendly error messages with parameter names and values
5. **Idiomatic**: Follow Rust's design philosophy and best practices

## Testing

```bash
cargo test --test argument_tests argument
```

## Documentation

Run `cargo doc --open` to view the complete API documentation.
