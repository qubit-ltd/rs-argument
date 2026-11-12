// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private implementation types used by argument validators.

// Implements shared duration comparisons and structured failure construction.
pub(super) mod duration_validation;
mod length_validation;
mod numeric_validation;
mod numeric_value;
mod sealed;
// Implements shared string validation without retaining inspected inputs.
pub(super) mod string_validation;

pub(super) use length_validation::validate_length;
pub(super) use numeric_validation::capture_range_constraint;
pub(super) use numeric_validation::range_contains;
pub(super) use numeric_validation::snapshot_range_bounds;
pub(super) use numeric_validation::validate_comparison;
pub(super) use numeric_validation::validate_not_nan;
pub(super) use numeric_validation::validate_range_structure;
pub(super) use numeric_value::NumericValue;
pub(super) use sealed::Sealed;
