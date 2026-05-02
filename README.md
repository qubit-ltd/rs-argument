# Qubit Argument

Argument and state validation helpers for Qubit Rust projects.

`qubit-argument` provides extension traits and functions for validating
numeric values, strings, collections, options, index bounds, and state
conditions.

## Features

- Numeric validation with range and equality checks.
- String validation for blank values, length constraints, and regex matching.
- Collection validation for length and element constraints.
- Option validation helpers.
- State and bounds checking functions.
- A small `ArgumentError` type and `ArgumentResult` alias.

## Installation

```toml
[dependencies]
qubit-argument = "0.1.0"
```

## Quick Start

```rust
use qubit_argument::{ArgumentResult, NumericArgument, StringArgument};

fn validate(age: i32, name: &str) -> ArgumentResult<()> {
    age.require_in_closed_range("age", 0, 150)?;
    name.require_non_blank("name")?;
    Ok(())
}
```

## Module Layout

- `argument`: validation traits, checking functions, `ArgumentError`, and
  `ArgumentResult`.

Top-level re-exports are provided for commonly used traits and functions.
