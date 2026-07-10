// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Reusable vocabulary describing argument validation constraints.

mod argument_bound;
mod comparison_constraint;
mod index_role;
mod length_constraint;
mod pattern_expectation;
mod range_constraint;

pub use argument_bound::ArgumentBound;
pub use comparison_constraint::ComparisonConstraint;
pub use index_role::IndexRole;
pub use length_constraint::LengthConstraint;
pub use pattern_expectation::PatternExpectation;
pub use range_constraint::RangeConstraint;
