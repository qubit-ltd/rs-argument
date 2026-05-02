# Qubit Argument

[![CircleCI](https://circleci.com/gh/qubit-ltd/rs-argument.svg?style=shield)](https://circleci.com/gh/qubit-ltd/rs-argument)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-argument/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-argument?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-argument.svg?color=blue)](https://crates.io/crates/qubit-argument)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Argument and state validation utilities for Rust.

## Overview

Qubit Argument provides extension traits and checking functions for validating
function arguments, configuration values, indexes, ranges, and runtime state.
It uses Rust's trait extension pattern for readable validation chains while
returning a small structured `ArgumentError`.

## Design Goals

- **Readable validation**: express checks close to the value being validated.
- **Small error surface**: use `ArgumentError` and `ArgumentResult` for all validation failures.
- **Method chaining**: return validated values or references so checks compose naturally.
- **No panic by default**: report invalid arguments through `Result`.
- **Focused scope**: provide validation utilities only, without pulling in broader common utilities.

## Features

### Numeric Validation

- Zero and non-zero checks.
- Positive, negative, non-positive, and non-negative checks.
- Closed, open, left-open, and right-open range validation.
- Less-than, less-or-equal, greater-than, and greater-or-equal checks.
- Equality and inequality validation across two named arguments.

### String Validation

- Non-blank checks.
- Exact length, minimum length, maximum length, and range length checks.
- Regular expression match and non-match checks.
- Implementations for both `str` and `String`.

### Collection and Option Validation

- Non-empty collection checks.
- Collection length constraints.
- Optional value presence checks.
- Conditional validation for present optional values.
- Element non-null checks for `&[Option<T>]`.

### State and Bounds Checking

- Boolean argument and state assertions.
- Slice-style bounds checks.
- Element index, position index, and position index range validation.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-argument = "0.1.0"
```

## Quick Start

### Numeric and String Validation

```rust
use qubit_argument::{
    ArgumentResult,
    NumericArgument,
    StringArgument,
};

fn validate_user(age: i32, username: &str) -> ArgumentResult<()> {
    age.require_in_closed_range("age", 0, 150)?;
    username
        .require_non_blank("username")?
        .require_length_in_range("username", 3, 20)?;
    Ok(())
}
```

### Collection Validation

```rust
use qubit_argument::{
    ArgumentResult,
    CollectionArgument,
};

fn validate_tags(tags: &[String]) -> ArgumentResult<&[String]> {
    tags.require_non_empty("tags")?
        .require_length_at_most("tags", 10)
}
```

### State and Bounds Checking

```rust
use qubit_argument::{
    ArgumentResult,
    check_bounds,
    check_state_with_message,
};

fn read_range(offset: usize, length: usize, total: usize) -> ArgumentResult<()> {
    check_state_with_message(total > 0, "total length must be positive")?;
    check_bounds(offset, length, total)
}
```

## API Reference

### Traits

- [`NumericArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.NumericArgument.html) - numeric validation methods.
- [`StringArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.StringArgument.html) - string validation methods.
- [`CollectionArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.CollectionArgument.html) - collection validation methods.
- [`OptionArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.OptionArgument.html) - optional value validation methods.

### Error Types

- [`ArgumentError`](https://docs.rs/qubit-argument/latest/qubit_argument/struct.ArgumentError.html) - validation error with a human-readable message.
- [`ArgumentResult`](https://docs.rs/qubit-argument/latest/qubit_argument/type.ArgumentResult.html) - result alias returned by validation APIs.

### Functions

- `check_argument`, `check_argument_with_message`, `check_argument_fmt`
- `check_state`, `check_state_with_message`
- `check_bounds`, `check_element_index`, `check_position_index`, `check_position_indexes`
- `require_equal`, `require_not_equal`, `require_element_non_null`, `require_null_or`

## Testing & Code Coverage

This project maintains comprehensive test coverage for success paths, failure
paths, boundary conditions, and representative type instantiations.

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Run CI checks (format, clippy, test, coverage, audit)
./ci-check.sh
```

### Coverage Metrics

See [COVERAGE.md](COVERAGE.md) for detailed coverage statistics.

## Dependencies

Runtime dependencies are intentionally small:

- `regex` powers regular expression validation for strings.

## License

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Follow the Rust API guidelines.
- Maintain comprehensive test coverage.
- Document all public APIs with examples when they clarify usage.
- Run `./ci-check.sh` before submitting PRs.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

## Related Projects

More Rust libraries from Qubit are published under the [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

---

Repository: [https://github.com/qubit-ltd/rs-argument](https://github.com/qubit-ltd/rs-argument)
