use glue::*;
use mpc_c::*;
use mpc_c_types::*;
#[macro_use]
use ::*;
#[test]
fn test_parse()
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
		Err(_) => println!("fuck") 
	}
}
