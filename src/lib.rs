#![allow(clippy::tabs_in_doc_comments)]
#![no_std]

pub mod gmod13;
pub mod source;

pub mod prelude;

#[cfg(doc)]

/// # Explanation of API errors in Rust binary modules
#[doc = include_str!("../doc/errors.md")]
pub mod errors {}
