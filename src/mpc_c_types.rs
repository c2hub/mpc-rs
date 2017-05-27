#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_void};

/*
** Aliases
*/
pub type parser_ptr = *mut mpc_parser_t;

/*
** State Type
*/

#[repr(C)]
pub struct mpc_state_t
{
	pub pos: i64,
	pub row: i64,
	pub col: i64,
}

/*
** Error Type
*/

#[repr(C)]
pub struct mpc_err_t
{
	pub state: mpc_state_t,
	pub expected_num: i32,
	pub filename: *const c_char,
	pub failure: *const c_char,
	pub expected: *const *const c_char,
}

/*
** Parsing Types
*/

pub type mpc_val_t = c_void;

// this is a union in C, it should look differently
// however, both members have the same size, weird.
#[repr(C)]
pub struct mpc_result_t
{
	pub output: *mut mpc_err_t
}


#[repr(C)]
pub struct mpc_parser_t;
pub type mpc_dtor_t = extern fn(val: *const mpc_val_t);
pub type mpc_ctor_t = extern fn() -> *const mpc_val_t;
pub type mpc_apply_t = extern fn() -> *const mpc_val_t;
pub type mpc_apply_to_t = extern fn(val: *const mpc_val_t, *const c_void) -> mpc_val_t;
pub type mpc_fold_t = extern fn(i: i32, v: *const *const mpc_val_t) -> mpc_val_t;

/*
** AST
*/

#[repr(C)]
pub struct mpc_ast_t
{
	pub tag: *const c_char,
	pub contents: *const c_char,
	pub state: mpc_state_t,
	pub children_num: i32,
	pub children: *mut *mut mpc_ast_t,
}

#[repr(u8)]
pub enum mpc_ast_trav_order_t
{
	mpc_ast_trav_order_pre,
	mpc_ast_trav_order_post,
}

#[repr(C)]
pub struct mpc_ast_trav_t
{
	pub curr_node: *const mpc_ast_t,
	pub parent: *const mpc_ast_trav_t,
	pub curr_child: i32,
	pub order: mpc_ast_trav_order_t,
}

#[repr(C)]
pub enum mpca_lang_type
{
	MPCA_LANG_DEFAULT = 0,
	MPCA_LANG_PREDICTIVE = 1,
	MPCA_LANG_WHITESPACE_SENSITIVE = 2,
}
