// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Private implementation types used by argument validators.

mod numeric_value;
mod sealed;

pub(super) use numeric_value::NumericValue;
pub(super) use sealed::Sealed;
