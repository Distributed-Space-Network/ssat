// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Core types for Move.

use std::fmt;

pub mod account_address;
pub mod annotated_extractor;
pub mod annotated_value;
pub mod annotated_visitor;
pub mod effects;
pub mod gas_algebra;
pub mod identifier;
pub mod language_storage;
pub mod metadata;
pub mod parsing;
#[cfg(any(test, feature = "fuzzing"))]
pub mod proptest_types;
pub mod resolver;
pub mod runtime_value;
pub mod u256;
#[cfg(test)]
mod unit_tests;
pub mod vm_status;

pub const VARIANT_COUNT_MAX: u64 = 127;

// Tags are zero-indexed so the max tag value is one less than the max variant count.
pub const VARIANT_TAG_MAX_VALUE: u64 = VARIANT_COUNT_MAX - 1;

pub(crate) fn fmt_list<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    begin: &str,
    items: impl IntoIterator<Item = T>,
    end: &str,
) -> fmt::Result {
    write!(f, "{}", begin)?;
    let mut items = items.into_iter();
    if let Some(x) = items.next() {
        write!(f, "{}", x)?;
        for x in items {
            write!(f, ", {}", x)?;
        }
    }
    write!(f, "{}", end)?;
    Ok(())
}
