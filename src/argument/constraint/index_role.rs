// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Index roles used by indexed argument validation.

/// The role of an index in an indexed argument operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexRole {
    /// Identifies an existing element and therefore excludes the collection
    /// end.
    Element,
    /// Identifies an insertion or boundary position, including the collection
    /// end.
    Position,
}
