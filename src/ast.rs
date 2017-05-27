#![allow(dead_code)]

use mpc_c_types::*;
use std::slice;

#[derive(Clone)]
pub struct Ast
{
	pub raw_ast: *mut mpc_ast_t,
	pub tag: String,
	pub contents: String,
	pub children: Vec<Ast>,
	pub row: usize,
	pub column: usize,
	pub position: usize,
}

#[derive(Clone)]
pub struct Child<'a>
{
	pub parent: &'a Ast,
	pub ast: &'a Ast,
	pub index: usize,
}


impl Ast
{
	pub fn new(ast_ptr: *mut mpc_ast_t) -> Ast
	{
		unsafe
		{
			let mut children: Vec<Ast> = Vec::new();

			for node in 
				slice::from_raw_parts((*ast_ptr).children, (*ast_ptr).children_num as usize)
			{
				children.push(Ast::new(*node));
			}

			Ast
			{
				raw_ast: ast_ptr,
				tag: dfs!(ast_ptr, tag),
				contents: dfs!(ast_ptr, contents),
				children: children,
				row: dfu!(ast_ptr, state.row),
				column: dfu!(ast_ptr, state.col),
				position: dfu!(ast_ptr, state.pos),
			}
		}
	}

	pub fn by_index(&self, index: usize) -> Option<Child>
	{
		if index < self.children.len()
		{
			Some(Child
			{
				parent: self,
				ast: &self.children[index],
				index: index,
			})
		}
		else { None }
	}

	pub fn by_tag(&self, tag: &str) -> Option<Child>
	{
		let children = self.children.clone().into_iter();
		let index: i32 =
			if let Some(_) =
				children.clone().filter(|x| x.tag == tag).next()
					{ children.count() as i32 }
			else { -1 };

		if index > -1
		{
			Some(Child
			{
				parent: self,
				ast: &self.children[index as usize],
				index: index as usize
			})
		}
		else { None }
	}

	pub fn by_contents(&self, contents: &str) -> Option<Child>
	{
		let children = self.children.clone().into_iter();
		let index: i32 =
			if let Some(_) =
				children.clone().filter(|x| x.contents == contents).next()
					{ children.count() as i32 }
			else { -1 };

		if index > -1
		{
			Some(Child
			{
				parent: self,
				ast: &self.children[index as usize],
				index: index as usize
			})
		}
		else { None }
	}
}
