//! This crate provides simple bindings for orangeduck's mpc library.
//! This is still a work in progress and contains a modified version of
//! mpc and some temporary glue code
#![feature(inclusive_range_syntax)]
#![deny(warnings, missing_docs)]

pub mod glue;
pub mod colors;

#[macro_use]
pub mod macros;

pub mod mpc_c;
pub mod ast;
pub mod mpc_c_types;

#[cfg(test)]
mod test;
