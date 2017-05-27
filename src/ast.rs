#![allow(dead_code)]

use mpc_c_types::*;
use std::slice;

pub struct Ast
{
	pub raw_ast: *mut mpc_ast_t,
	pub tag: String,
	pub contents: String,
	pub children: Vec<Ast>,
}

pub struct Child
{
	pub parent: Ast,
	pub ast: Ast,
	pub index: i32,
}


impl Ast
{
	pub fn new(ast_ptr: *mut mpc_ast_t) -> Ast
	{
		unsafe
		{
			let tag = dfs!(ast_ptr, tag);
			let contents = dfs!(ast_ptr, contents);
			let mut children: Vec<Ast> = Vec::new();

			for node in 
				slice::from_raw_parts((*ast_ptr).children, (*ast_ptr).children_num as usize)
			{
				children.push(Ast::new(*node));
			}

			Ast
			{
				raw_ast: ast_ptr,
				tag: tag,
				contents: contents,
				children: children,
			}
		}
	}
}
