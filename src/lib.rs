#![feature(inclusive_range_syntax)]

mod glue;
mod colors;

#[macro_use]
mod macros;

mod mpc_c;
mod ast;
mod mpc_c_types;

#[cfg(test)]
mod test;
