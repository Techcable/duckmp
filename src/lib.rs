#![feature(
    unchecked_math, // Gotta go fast
    never_type, // Cleaner than `enum Empty {}`
)]
#![deny(missing_docs)]
#![deny(clippy::missing_safety_doc)]
//! A fast, liberally licensed multiple precision
//! arithmetic library.
//!
//! Only implements integer arithmetic.

pub mod arith_utils;
pub mod uint;
pub mod memory;
mod string;