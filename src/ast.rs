//! Contains an AST type to which `mpc_ast_t` is mapped
#![allow(dead_code)]

use colors::*;
use mpc_c_types::*;
use std::slice;

/// The Ast type
#[derive(Clone, Eq, PartialEq)]
pub struct Ast
{
	/// Pointer to the underlying Ast
	/// It is not recommended to modify it
	pub raw_ast: *mut mpc_ast_t,
	/// Tag of this node. Folded tags are connected with
	/// the pipe character `|`, tags of nodes with children
	/// end with `|>`.
	pub tag: String,
	/// Contents of this node
	pub contents: String,
	/// A vector containing children of this node
	pub children: Vec<Ast>,
	/// Number of the row where this node is located, only valid
	/// for nodes without children
	pub row: usize,
	/// Number of the column where this node is located, only valid
	/// for nodes without children
	pub column: usize,
	/// Total position in the file, only valid for nodes with no children
	pub position: usize,
}

/// A temporary type to represent a node returned
/// by one of the searching functions
#[derive(Clone, Eq, PartialEq)]
pub struct Child<'a>
{
	/// The parent node
	pub parent: &'a Ast,
	/// Ast of the current node
	pub ast: &'a Ast,
	/// Index of this node in the parent node
	pub index: usize,
}


impl Ast
{
	/// Create a new Ast from a raw `mpc_ast_t`
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

	/// Find a child by index
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

	/// Find a child by tag
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

	/// Find a child by contents
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

	/// Print the Ast
	pub fn print(&self)
	{
		self.print_level(0);
	}

	/// Print the Ast with starting depth (indentation) `level`
	pub fn print_level(&self, level: usize)
	{
		for _ in 0..level {trace!("  ");}
		if self.children.is_empty()
		{
			trace!(
				RED self.tag
				RESET ':'
				GREEN self.row
				RESET ':'
				YELLOW self.column " "
				RESET '\''
				self.contents "'\n"
			);
		}
		else
		{
			trace!(
				MAGENTA self.tag
				RESET "\n"
			);
			for child in &self.children
			{
				child.print_level(level+1);
			}
		}
	}
}
