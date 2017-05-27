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
			traceln!("\nlen:" ast.children.len());
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
fn delayed_parse()
{
	let parsers = parser!
	{
		grammar:
		{g_string![
			"word : /[a-zA-Z0-9]+/;                         \n"
			"punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
			"sentence: <word>+ <punct>;                     \n"
			"paragraph: <sentence>+;                        \n"
		]}
		main: paragraph
		parsers: word punct sentence
	};

	let result = run_parser!
	{
		preparsers: parsers
		input:
		{
			"A big brown piece of DOG jumped over something.".to_string() +
			"Bananas are awesome. What do you mean, potato?"
		}
	};

	match result
	{
		Ok(a) =>
		{
			let ast: Ast = Ast::new(a);
			traceln!("\nlen:" ast.children.len());
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

#[test]
fn c2_parse()
{
	let parsers = parser!
	{
		grammar:
		{g_string![
			/*************************************************************************************************************
			*============================================================================================================*
			*----------------------------------------------BASIC PARSERS-------------------------------------------------*
			*============================================================================================================*
			*************************************************************************************************************/
			" start                             : /^/ ;                                                                \n"
			" end                               : /$/ ;                                                                \n"
			" ptrop        \"Pointer operator\" : '*' ;                                                                \n"
			" uop                               :  '&' | '*' | '+' | '-' | '~' | '!'  ;                                \n"
			" asop      \"Assignment operator\" :( \"=\"  | \"+=\" | \"-=\" | \"*=\" | \"/=\"                          \n"
			"                                   |  \"&=\" | \"|=\" | \"~=\" | \"<<=\"| \">>=\" );                      \n"
			" ident              \"Identifier\" : /[a-zA-Z_][a-zA-Z0-9_]*/ ;                                           \n"
			" symbol                 \"Symbol\" : (<ident> '.')? <ident> ;                                             \n"
			" integer               \"Integer\" : /(0x)?[0-9]+/ ;                                                      \n"
			" character       \"Any character\" : '\'' /\\\\?[^\\n']*/ '\'' ;                                          \n"
			" stringlit     \"String literal\" : (/\"(\\\\.|[^\"])+\"/)+ ;                                             \n"
			" public                            : (\"public\")? ;                                                      \n"
			" floatn   \"Floating-point value\" : /[0-9]+\\.[0-9]+[a-zA-Z]*/ ;                                         \n"
			" natives           \"Native type\" : \"void\"                                                             \n"
			"                                   | \"char\"    | \"bool\"                                               \n"
			"                                   | \"int8\"    | \"uint8\"                                              \n"
			"                                   | \"int16\"   | \"uint16\"                                             \n"
			"                                   | \"int32\"   | \"uint32\"                                             \n"
			"                                   | \"int64\"   | \"uint64\"                                             \n"
			"                                   | \"float32\" | \"float64\" ;                                          \n"
			" index                   \"Index\" : '[' ( '+' | <exp>)? ']' ;                                            \n"
			" number                 \"Number\" :  <floatn> | <integer>  ;                                             \n"
			" constant             \"Constant\" :  \"nil\" | <number> | <stringlit> | <ident>  ;                       \n"
	    	/*************************************************************************************************************
			*============================================================================================================*
			*%%%%%%%%%%%%%%%/------------------------------------------------------------------------------\%%%%%%%%%%%%%*
			*%%%%%%%%%%%%%%%|                           Here lie EXPRESSIONS                               |%%%%%%%%%%%%%*
			*%%%%%%%%%%%%%%%|                          Pain in the ass to make,                            |%%%%%%%%%%%%%*
			*%%%%%%%%%%%%%%%|                will eat your dog if you try to modify it                     |%%%%%%%%%%%%%*
			*%%%%%%%%%%%%%%%\------------------------------------------------------------------------------/%%%%%%%%%%%%%*
			*============================================================================================================*
			*Hours wasted: 9;                                                                       --Lukáš Hozda, 2017  *
			*************************************************************************************************************/
			" pexp                              : <ident> | <number> | <stringlit> | <character> |'(' <exp> ')' ;      \n"
			" pfexp                             : <pexp>                                                               \n"
			"                                   ( <params>                                                             \n"
			"                                   | '[' <exp> ']'                                                        \n"
			"                                   | '[' <integer> ':' <integer> ']'                                      \n"
			"                                   | '.' <ident>                                                          \n"
			"                                   | (\"++\"|\"--\")                                                      \n"
			"                                   )* ;                                                                   \n"
			" uexp                              : <pfexp>                                                              \n"
			"                                   | (\"++\"|\"--\") <uexp>                                               \n"
			"                                   | <uop> <cast>                                                         \n"
			"                                   | (\"sizeof\"|\"elemsof\") ( <uexp> | '(' <c2type> ')' ) ;             \n"
			" cast                              : ( \"(->\" <c2type> ')' )? <uexp> ;                                   \n"
			" mexp                              : <cast> (('*'|'/'|'%') <cast>)* ;                                     \n"
			" aexp                              : <mexp> (('+'|'-') <mexp>)* ;                                         \n"
			" sexp                              : <aexp> (( \"<<\" | \">>\" ) <aexp>)* ;                               \n"
			" rexp                              : <sexp> (( \"<=\" | \">=\" | \"<\" | \">\" ) <sexp>)* ;               \n"
			" eexp                              : <rexp> ((\"==\"|\"!=\") <rexp>)* ;                                   \n"
			" bexp                              : <eexp> (('|'|'^'|'&') <eexp>)* ;                                     \n"
			" lexp                              : <bexp> ((\"&&\"|\"||\") <bexp>)* ;                                   \n"
			" elexp                             : <lexp> ('?' <lexp> ':' <lexp>)* ;                                    \n"
			" asexp                             : <elexp> (<asop> <asexp>)* ;                                          \n"
			" exp                \"Expression\" : <asexp> (',' <asexp> )* ;                                            \n"
			" params                            : '(' (<elexp> (',' <elexp>)*)? ')' ;                                  \n"
	    	/*************************************************************************************************************
			*============================================================================================================*
			*---------------------------------------------FUNCTION STUFF-------------------------------------------------*
			*============================================================================================================*
			*************************************************************************************************************/
			" arg                               : <c2type> (<ident> ('=' <constant>)?)? ;                              \n"
			" args      \"Function parameters\" : '(' (<arg> (',' <arg>)*)* (',' \"...\")? ')' ;                       \n"
			" label                   \"Label\" : \"case\" <elexp> ':' <stmt>                                          \n"
			"                                   | \"default\" ':' <stmt>                                               \n"
			"                                   | <ident> ':' <stmt> ;                                                 \n"
			" expstmt  \"Expression statement\" : <exp> ';' ;                                                          \n"
			" compound   \"Compound statement\" : '{' <stmt>* '}' ;                                                    \n"
			" branch    \"Branching statement\" : \"if\" '(' <exp> ')' <stmt> (\"else\" <stmt>)?                       \n"
			"                                   | \"switch\" '(' <exp> ')' <stmt> ;                                    \n"
			" iter      \"Iterating statement\" : \"while\" '(' <exp> ')' <stmt>                                       \n"
			"                                   | \"do\" <stmt> \"while\" '(' <exp> ')' ';'                            \n"
			"                                   | \"for\" '(' (<stmt>|';') (<stmt>|';') <exp>? ')' <stmt> ;            \n"
			" jump   \"Flow control statement\" : \"goto\" <ident> ';'                                                 \n"
			"                                   | \"continue\" ';'                                                     \n"
			"                                   | \"break\" ';'                                                        \n"
			"                                   | \"return\" <exp>? ';' ;                                              \n"
			" declstmt\"Declaration statement\" : (\"local\")? <c2type> <ident> ( ('=' <exp>)? ';' | ('=' <init>) )? ; \n"
			" stmt                              : <label>                                                              \n"
			"                                   | <jump>                                                               \n"
			"                                   | <expstmt>                                                            \n"
			"                                   | <compound>                                                           \n"
			"                                   | <declstmt>                                                           \n"
			"                                   | <iter>                                                               \n"
			"                                   | <branch> ;                                                           \n"
			" func                 \"Function\" : <public> \"func\" <c2type> <ident> <args> <attribute>? <compound> ;  \n"
	    	/*************************************************************************************************************
			*============================================================================================================*
			*--------------------------------------------------TYPES-----------------------------------------------------*
			*============================================================================================================*
			*************************************************************************************************************/
			" c2type                   \"Type\" : (\"const\"|\"volatile\")? (<natives>|<symbol>) <ptrop>* <index>* ;   \n"
			" member                 \"Member\" : <structlet> | <uniontype> | (<c2type> <ident> (':' <integer>)? ';') ;\n"
			" memberblock           \"Members\" : '{' <member>* '}' ;                                                  \n"
			" alias                   \"Alias\" : <ident> <c2type> ';' ;                                               \n"
			" uniontype               \"Union\" : \"union\" <ident>? <memberblock> ;                                   \n"
			" structlet              \"Struct\" : \"struct\" <ident>? <memberblock> <attribute>? ;                     \n"
			" globalunion        \"Union type\" : <ident> \"union\" <memberblock> ;                                    \n"
			" functype        \"Function type\" : <ident> \"func\" <c2type> <args> <attribute>? ';' ;                  \n"
			" structure         \"Struct Type\" : <ident> \"struct\" <memberblock> <attribute>? ;                      \n"
			" enumeration                       : '{' <ident> ('='<integer>)? (',' <ident> ('='<integer>)?)* ','? '}' ;\n"
			" enumtype          \"Enumeration\" : <ident> \"enum\" <c2type> <enumeration> <attribute>? ;               \n"
			" usertype    \"User-defined type\" : <public> \"type\" ( <structure>   | <enumtype>                       \n"
			"                                                       | <globalunion> | <functype>  | <alias> ) ;        \n"
	    	/*************************************************************************************************************
			*============================================================================================================*
			*----------------------------------------------DECLARATIONS--------------------------------------------------*
			*============================================================================================================*
			*************************************************************************************************************/
			" vardecl  \"Variable declaration\" : <c2type> <ident> ('=' <exp>)? ';' ;                                  \n"
			" init                              : '{'((('.'<ident>|'['<constant>']') '=')? (<init>|<elexp>)            \n"
			"                                    (','(('.'<ident>|'['<constant>']') '=')?(<init>|<elexp>))* )?','? '}';\n"
			" cmpddecl \"Compound declaration\" : <c2type> <ident> '=' <init> ;                                        \n"
			" decl              \"Declaration\" : <public> (<vardecl> | <cmpddecl>) ;                                  \n"
			" arrayincr     \"Array increment\" : <symbol> \"+=\" (<exp> ';'|<init>) ;                                 \n"
	    	/*************************************************************************************************************
			*============================================================================================================*
			*------------------------------------------------ATTRIBUTE---------------------------------------------------*
			*============================================================================================================*
			*************************************************************************************************************/
			" attrtype                          : \"export\"   | \"packed\"                                            \n"
			"                                   | \"unused\"   | \"unused_param\"                                      \n"
			"                                   | \"noreturn\" | \"inline\"                                            \n"
			"                                   | \"weak\"     | \"opaque\" ;                                          \n"
			" attrparam                         : (\"section\" | \"aligned\") '=' (<stringlit>|<integer>) ;            \n"
			" attribute                         : \"@(\"(<attrtype>|<attrparam>) (',' (<attrtype>|<attrparam>))* ')' ; \n"
	        /*************************************************************************************************************
			*============================================================================================================*
			*-----------------------------------------------FILE STUFF---------------------------------------------------*
			*============================================================================================================*
			*************************************************************************************************************/
			" module                 \"Module\" : \"module\" <ident> ';' ;                                             \n"
			" import                 \"Import\" : \"import\" <ident> (\"as\" <ident>)? (\"local\")? ';' ;              \n"
			" head                              : <module> <import>* ;                                                 \n"
			" body                              : (<usertype> | <func> | <decl> | <arrayincr> )* ;                     \n"
			" c2                                : <start> <head> <body> <end> ;                                        \n"
		]}
		main: c2
		parsers: start end ptrop ident symbol integer character
		         stringlit public floatn natives index number
		         constant pexp pfexp params cast uexp uop mexp
		         aexp sexp rexp eexpbexp lexp elexp asexp asop
		         cexp exp arg args label expstmt declstmt compound
		         branch iter jump stmt func member memberblock
		         c2type alias uniontype structlet globalunion
		         functype structure enumeration enumtype usertype
		         vardecl init cmpddecl decl arrayincr attrtype
		         attrparam attribute module import head body
	};

	let raw: Vec<char> = include_str!("../inc/test.c2").chars().collect();
	let mut contents = String::new();

	enum ReadState
	{
		SingleComment,
		MultiComment,
		NoComment
	}

	let mut state = ReadState::NoComment;

	//we meet again, old friend, comment skip
	for i in 0..raw.len()
	{
		if raw[i] == '\r' {continue;}
		else if i != 0 && raw[i-1...i] == ['\r', '\n'] { contents.push('\n'); }
		match state
		{
			ReadState::SingleComment => if raw[i] == '\n'
			{
				state = ReadState::NoComment;
				contents.push(raw[i]);
			},
			ReadState::MultiComment => if &raw[i-1...i] == ['*','/']
				{state = ReadState::NoComment;},
			ReadState::NoComment =>
			{
				if i+1 < raw.len()
				{
					if &raw[i...i+1] == ['/','/'] { state = ReadState::SingleComment; }
					else if &raw[i...i+1] == ['/','*'] { state = ReadState::MultiComment; }
					else { contents.push(raw[i]); }
				}
			}
		}
	}

	traceln!(contents);

	let result = run_parser!
	{
		preparsers: parsers
		filename: {"test.c2"}
		input: { contents }
	};

	match result
	{
		Ok(a) =>
		{
			let ast: Ast = Ast::new(a);
			traceln!("\nlen:" ast.children.len());
			traceln!("tag:" ast.tag);
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
