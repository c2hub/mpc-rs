use glue;
use ast::*;
use mpc_c::*;
use mpc_c_types::*;

#[test]
fn mpca_parse()
{
	let result = parser!
	{
		grammar:
		{g_string![
			"word : /[a-zA-Z0-9]+/;                         \n"
			"punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
			"sentence: <word>+ <punct>;                     \n"
			"paragraph: <sentence>+;                        \n"
		]}
		filename: {"test.txt"}
		input:
		{
			"A big brown piece of DOG jumped over something.".to_string() +
			"Bananas are awesome. What do you mean, potato?"
		}
		main: paragraph
		parsers: word punct sentence
	};

	match result
	{
		Ok(ast) =>
		{
			println!("success!");
			unsafe { mpc_ast_print(ast); }
		},
		Err(r) =>
		{
			println!("fail!");
			unsafe { mpc_err_print(r); }
			panic!();
		}
	}
}

#[test]
fn make_ast()
{
	let result = parser!
	{
		grammar:
		{g_string![
			"word : /[a-zA-Z0-9]+/;                         \n"
			"punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
			"sentence: <word>+ <punct>;                     \n"
			"paragraph: <sentence>+;                        \n"
		]}
		filename: {"test.txt"}
		input:
		{
			"A big brown piece of DOG jumped over something.".to_string() +
			"Bananas are awesome. What do you mean, potato?"
		}
		main: paragraph
		parsers: word punct sentence
	};

	match result
	{
		Ok(a) =>
		{
			let ast: Ast = Ast::new(a);
			traceln!("len:" ast.children.len());
			traceln!("tag:" ast.tag);
		},
		Err(r) =>
		{
			println!("fail!");
			unsafe { mpc_err_print(r); }
			panic!();
		}
	}
}

#[test]
fn print_ast()
{
	let result = parser!
	{
		grammar:
		{g_string![
			"word : /[a-zA-Z0-9]+/;                         \n"
			"punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
			"sentence: <word>+ <punct>;                     \n"
			"paragraph: <sentence>+;                        \n"
		]}
		filename: {"test.txt"}
		input:
		{
			"A big brown piece of DOG jumped over something.".to_string() +
			"Bananas are awesome. What do you mean, potato?"
		}
		main: paragraph
		parsers: word punct sentence
	};

	match result
	{
		Ok(a) =>
		{
			let ast: Ast = Ast::new(a);
			ast.print();
		},
		Err(r) =>
		{
			println!("fail!");
			unsafe { mpc_err_print(r); }
			panic!();
		}
	}
}

#[test]
fn manual_parse()
{
	unsafe
	{
		if let Err(_) = glue::parse(c_str!("test text"), c_str!("a"), mpc_alpha())
		{
			panic!()
		}
	}
}
