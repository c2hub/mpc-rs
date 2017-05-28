//! Contains a Rust representation of mpc's underlying C types
//! that are exposed in the `mpc.h` header
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_void};

/*
** Aliases
*/
/// Shorthand for an ast pointer
pub type parser_ptr = *mut mpc_parser_t;

/*
** State Type
*/

/// Contains information about position in a file
#[repr(C)]
pub struct mpc_state_t
{
	/// Total position from the start of the file
	pub pos: i64,
	/// Number of row in the file
	pub row: i64,
	/// Number of column in the file
	pub col: i64,
}

/*
** Error Type
*/

/// mpc's error type
#[repr(C)]
pub struct mpc_err_t
{
	/// Contains state information
	pub state: mpc_state_t,
	/// Number of expected items
	pub expected_num: i32,
	/// Filename of the erroneous file
	pub filename: *const c_char,
	/// Failure text
	pub failure: *const c_char,
	/// Array of expected items
	pub expected: *const *const c_char,
}

impl mpc_err_t
{
	/// Create a new mpc_err_t
	pub fn new(failure: &str) -> mpc_err_t
	{
		mpc_err_t
		{
			state: mpc_state_t
			{
				pos: 0,
				row: 0,
				col: 0,
			},
			expected_num: 0,
			filename: c_str!("<input>"),
			failure: c_str!(failure),
			expected: 0 as *const *const c_char
		}
	}
}

/*
** Parsing Types
*/
/// mpc_val_t is an alias to void in C as well
pub type mpc_val_t = c_void;

/// This is a union in C, it should look differently.
/// Both members have the same size, since both are pointers.
#[repr(C)]
pub struct mpc_result_t
{
	/// Pointer to either error or ast,
	/// terribly unsafe
	/// std::mem::transmute for turning one into another
	/// the only indicator what is the underlying type
	/// is `mpc_parse()`'s return value:
	///
	/// 1 => *mut mpc_ast_t
	///
	/// 0 => *mut mpc_err_t
	pub output: *mut mpc_err_t
}

/// Parser type, implementation is hidden and very complex
/// for me to represent it in Rust.
/// Besides, it is really only needed to be used as a pointer
#[repr(C)]
pub struct mpc_parser_t;
/// Destructor function type
pub type mpc_dtor_t = extern fn(val: *const mpc_val_t);
/// Constructor function type
pub type mpc_ctor_t = extern fn() -> *const mpc_val_t;
/// Apply function type
pub type mpc_apply_t = extern fn() -> *const mpc_val_t;
/// 'Apply to' function type
pub type mpc_apply_to_t = extern fn(val: *const mpc_val_t, *const c_void) -> mpc_val_t;
/// Fold function type
pub type mpc_fold_t = extern fn(i: i32, v: *const *const mpc_val_t) -> mpc_val_t;

/*
** AST
*/

/// Representation of mpc's ast type in Rust
#[repr(C)]
pub struct mpc_ast_t
{
	/// Tag of the node
	pub tag: *const c_char,
	/// Contents of the node
	pub contents: *const c_char,
	/// State (read position) of the node
	pub state: mpc_state_t,
	/// Number of child nodes
	pub children_num: i32,
	/// Child nodes
	pub children: *mut *mut mpc_ast_t,
}

/// Traversal order
#[repr(u8)]
pub enum mpc_ast_trav_order_t
{
	/// From beginning to end
	mpc_ast_trav_order_pre,
	/// From end to beginning
	mpc_ast_trav_order_post,
}

/// Traversal types
#[repr(C)]
pub struct mpc_ast_trav_t
{
	/// Current node
	pub curr_node: *const mpc_ast_t,
	/// Parent traversal
	pub parent: *const mpc_ast_trav_t,
	/// Index of current child
	pub curr_child: i32,
	/// mpc traversal order
	pub order: mpc_ast_trav_order_t,
}

/// mpca_lang grammar types
#[repr(C)]
pub enum mpca_lang_type
{
	/// Default mpca lang type
	MPCA_LANG_DEFAULT = 0,
	/// Predictive lang type
	MPCA_LANG_PREDICTIVE = 1,
	/// Whitespace sensitive lang type
	MPCA_LANG_WHITESPACE_SENSITIVE = 2,
}
