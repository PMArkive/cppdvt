//! Set of utilities for working with C++ virtual function tables.
//! 
//! This crate does not use any proc macros; all of the macros use `macro_rules!`.

#![no_std]
#![allow(clippy::tabs_in_doc_comments)]

#[cfg(feature = "macros")]
mod macros;

mod vtable_ptr;
pub use vtable_ptr::*;
mod vt_object;
pub use vt_object::*;
