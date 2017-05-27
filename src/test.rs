use glue;
use mpc_c::*;
use mpc_c_types::*;

#[test]
fn mpca_parse()
{
	let result = parser!
	{
	   "word : /[a-zA-Z0-9]+/;
		punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';
		sentence: <word>+ <punct>;
		paragraph: <sentence>+;",
		"A big brown piece of shit jumped over something. Bananas are awesome. What do you mean, bitch?",
		"test.txt",
		paragraph, word punct sentence
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
