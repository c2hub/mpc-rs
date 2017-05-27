#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

use mpc_c_types::*;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Eq, PartialEq)]
pub enum res_t
{
	ok,
	err
}

#[repr(C)]
pub struct parse_result
{
	pub ok: *mut mpc_ast_t,
	pub err: *mut mpc_err_t,
	pub res: res_t,
}

extern
{
	pub fn glue_parse(filename: *const c_char, string: *const c_char, p: *mut mpc_parser_t) -> parse_result;
}

pub fn parse(filename: *const c_char, string: *const c_char, p: *mut mpc_parser_t) -> Result<*mut mpc_ast_t, *mut mpc_err_t>
{
	let res = unsafe { glue_parse(filename, string, p) };

	match res.res
	{
		res_t::ok => Ok(res.ok),
		res_t::err => Err(res.err),
	}
}
