//! Contains the rust-side of glue code.
//! The main purpose of the code is to avoid having to
//! interface with a C union in Rust. The main offender
//! is `mpc_result_t`
#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

use mpc_c_types::*;
use std::os::raw::c_char;


/// Determines whether compilation was successful
#[repr(C)]
#[derive(Eq, PartialEq)]
pub enum res_t
{
	/// successful parse returning `mpc_ast_t` pointer
	ok,
	/// unsuccessful parse returning `mpc_err_t` pointer
	err
}

/// Contains resulting data from parsing
/// this struct would be a book example of where to use unions,
/// unions in Rust, however...
#[repr(C)]
pub struct parse_result
{
	/// contains a pointer to resulting AST when `res` is `ok`
	pub ok: *mut mpc_ast_t,
	/// contains a pointer to resulting error when `res` is `err`
	pub err: *mut mpc_err_t,
	/// `ok` when parsing was successful, `err` when not
	pub res: res_t,
}

extern
{
	/// The C function which handles parsing
	pub fn glue_parse(filename: *const c_char, string: *const c_char, p: *mut mpc_parser_t) -> parse_result;
}

/// Convert parse_result to `Result<*mut mpc_ast_t, *mut mpc_err_t>`. It is more idiomatic
pub fn parse(filename: *const c_char, string: *const c_char, p: *mut mpc_parser_t) -> Result<*mut mpc_ast_t, *mut mpc_err_t>
{
	let res = unsafe { glue_parse(filename, string, p) };

	match res.res
	{
		res_t::ok => Ok(res.ok),
		res_t::err => Err(res.err),
	}
}
