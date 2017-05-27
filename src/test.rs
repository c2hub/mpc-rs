use glue;
use mpc_c::*;
use mpc_c_types::*;

#[test]
fn mpca_parse()
{
	let result = parser!
	{
		grammar:
		{
			"word : /[a-zA-Z0-9]+/;                         \n".to_string() +
			"punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n" +
			"sentence: <word>+ <punct>;                     \n" +
			"paragraph: <sentence>+;                        \n"
		}
		filename: {"test.txt"}
		input:
		{
			"A big brown piece of shit jumped over something.".to_string() +
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
