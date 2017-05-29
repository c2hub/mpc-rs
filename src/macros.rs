//! Contains some nice macros to help with needed FFI and parser creation
//! The str_c! and c_str! macros exist because ownership issues

/// Convert a rustic string (whether `&str` or `String`) to C's `const char*`
#[macro_export]
macro_rules! c_str /* C-ish String */
{
	($str:expr) =>
	{{
		use std::ffi::CString;
		CString::new($str).unwrap().as_ptr()
	}}
}

/// Convert a C's `const char*` to a rustic `String`
#[macro_export]
macro_rules! str_c /* String from C */
{
	($ptr:expr) =>
	{{
		use std::ffi::CStr;
		CStr::from_ptr($ptr).to_string_lossy().into_owned()
	}}
}

/// Dereference pointer and convert struct member `mem` to `String`
#[macro_export]
macro_rules! dfs /* dereferenced's member to String */
{
	($ptr:expr, $mem:ident) =>
	{{
		str_c!((*$ptr).$mem)
	}}
}

/// Dereference pointer and convert struct member `mem` to `usize`
#[macro_export]
macro_rules! dfu /* dereferenced's member to usize */
{
	($ptr:expr, $mem:ident) =>
	{{
		(*$ptr).$mem as usize
	}};
	($ptr:expr, $mem1:ident . $mem2:ident) =>
	{{
		(*$ptr).$mem1.$mem2 as usize
	}}
}

/// `print!` for lazy people - no format, args are thrown in delimited with spaces
#[macro_export]
macro_rules! trace
{
	($arg:expr) =>
	{{
		print!("{}", $arg);
	}};
	($($arg:expr)+) =>
	{{$(
		print!("{}", $arg);
	)+}};
}

/// `println!` for lazy people - no format, args are thrown in delimited with spaces
/// also throw in a newline
#[macro_export]
macro_rules! traceln
{
	($arg:expr) =>
	{{
		trace!($arg '\n');
	}};
	($($arg:expr)+) =>
	{{
		trace!( $( $arg )+ '\n');
	}}
}

/// simulate C's compile time string literal concatenation to allow copy-pasta of mpc
/// grammars written in C without any hassle
/// # Usage
///
/// ```rust
/// # #[macro_use] extern crate mpc;
/// # fn main() {
/// let _ = g_string![
///      "So many strings\n"
///      "So many options\n"
///      "So much things\n"
/// ];
/// # }
/// ```
#[macro_export]
macro_rules! g_string /* couldn't help it */
{
	($($arg:expr)+) =>
	{{
		concat!( $( $arg, )+ )
	}}
}

/// Create a `mpc_parser_t` using `mpca_lang` grammars
/// # Usage
/// There are four possible ways to use this macro. Each
/// does a slightly different thing:
/// 1. To make a parser and immediately run on provided input
/// in the form of something stringy:
///
/// ```rust
/// # #[macro_use] extern crate mpc;
/// # #[allow(unused_variables)]
/// # fn main() {
/// let my_result = parser!
/// {
///     grammar:{g_string![
///	         "word : /[a-zA-Z0-9]+/;                         \n"
///	         "punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
///	         "sentence: <word>+ <punct>;                     \n"
///	         "paragraph: <sentence>+;                        \n"
///     ]}
///     filename: {"myfilename.txt"}
///     input: { "A big brown piece of DOG jumped over something.".to_string() +
///             "Bananas are awesome. What do you mean, potato?"}
///     main: paragraph
///     parsers: word punct sentence
/// };
/// # }
/// ```
///
/// 2. To do the same, but automatically read the file:
///
/// ```no_run
/// # #[macro_use] extern crate mpc;
/// # fn main() {
/// let my_result = parser!
/// {
///     grammar:{g_string![
///	         "word : /[a-zA-Z0-9]+/;                         \n"
///	         "punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
///	         "sentence: <word>+ <punct>;                     \n"
///	         "paragraph: <sentence>+;                        \n"
///     ]}
///     filename: {"myfilename.txt"}
///     main: paragraph
///     parsers: word punct sentence
/// };
/// # }
/// ```
///
/// 3. Parse input without a filename:
///
/// ```rust
/// # #[macro_use] extern crate mpc;
/// # #[allow(unused_variables)]
/// # fn main() {
/// let my_result = parser!
/// {
///     grammar:{g_string![
///	         "word : /[a-zA-Z0-9]+/;                         \n"
///	         "punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
///	         "sentence: <word>+ <punct>;                     \n"
///	         "paragraph: <sentence>+;                        \n"
///      ]}
///      input: { "A big brown piece of DOG jumped over something.".to_string() +
///               "Bananas are awesome. What do you mean, potato?"}
///      main: paragraph
///      parsers: word punct sentence
/// };
/// # }
/// ```
///
/// 4. Prepare parsers for later use:
///
/// ```rust
/// # #[macro_use] extern crate mpc;
/// # #[allow(unused_variables)]
/// # fn main() {
/// let my_parser = parser!
/// {
///     grammar:{g_string![
///	         "word : /[a-zA-Z0-9]+/;                         \n"
///	         "punct: '.' | '!' | ',' | ';' | '?' | '-' | ':';\n"
///	         "sentence: <word>+ <punct>;                     \n"
///	         "paragraph: <sentence>+;                        \n"
///     ]}
///     main: paragraph
///     parsers: word punct sentence
/// };
/// # }
/// ```
/// For cases 1-3 `parser!` returns `Result<*mut mpc_ast_t, *mut mpc_err_t>`
/// In case 4 `parser!` returns a vector containing prepared parsers. The vector
/// is not to be touched by a programmer. Its sole purpose is to be passed to
/// `run_parser!`
#[macro_export]
macro_rules! parser
{
	/* gimme everything variant */
	(grammar: {$grammar:expr}
	 filename: {$filename:expr}
	 input: {$input:expr}
	 main: $top:ident
	 parsers: $($p:ident)+) =>
	{{ unsafe {
		use mpc::glue;
		use mpc::mpc_c::*;
		use mpc::mpc_c_types::*;
		use std::os::raw::c_void;

		let $top = mpc_new(c_str!(stringify!($top)));
		$
		(
			let $p = mpc_new(c_str!(stringify!($p)));
		)+

		mpca_lang(
			mpca_lang_type::MPCA_LANG_DEFAULT,
			c_str!($grammar)
			$(, $p)+,
			$top,
			0 as *mut c_void
		);

		glue::parse(
			c_str!($filename),
			c_str!($input),
			$top,
		)
	}}};
	/* read the file myself variant */
	(grammar: {$grammar:expr}
	 filename: {$filename:expr}
	 main: $top:ident
	 parsers: $($p:ident)+) =>
	{{ unsafe {
		use mpc::glue;
		use mpc::mpc_c::*;
		use mpc::mpc_c_types::*;
		use std::io::Read;
		use std::os::raw::c_void;
		use std::fs::File;

		let mut input = String::new();
		if let Ok(mut ay) = File::open($filename)
		{
			let _ = ay.read_to_string(&mut input);
		};
		if input == String::new()
		{
			panic!();
		}
		let $top = mpc_new(c_str!(stringify!($top)));
		$
		(
			let $p = mpc_new(c_str!(stringify!($p)));
		)+

		mpca_lang(
			mpca_lang_type::MPCA_LANG_DEFAULT,
			c_str!($grammar)
			$(, $p)+,
			$top,
			0 as *mut c_void
		);

		glue::parse(
			c_str!($filename),
			c_str!(input),
			$top,
		)
	}}};
	/* no filename variant */
	(grammar: {$grammar:expr}
	 input: {$input:expr}
	 main: $top:ident
	 parsers: $($p:ident)+) =>
	{{ unsafe {
		use mpc::glue;
		use mpc::mpc_c::*;
		use mpc::mpc_c_types::*;
		use std::os::raw::c_void;
		let $top = mpc_new(c_str!(stringify!($top)));
		$
		(
			let $p = mpc_new(c_str!(stringify!($p)));
		)+

		mpca_lang(
			mpca_lang_type::MPCA_LANG_DEFAULT,
			c_str!($grammar)
			$(, $p)+,
			$top,
			0 as *mut c_void
		);

		glue::parse(
			c_str!("<input>"),
			c_str!($input),
			$top,
		)
	}}};
	/* prepare parsers for later use */
	(grammar: {$grammar:expr}
	 main: $top:ident
	 parsers: $($p:ident)+) =>
	{{ unsafe {
		use mpc::mpc_c::*;
		use mpc::mpc_c_types::*;
		use std::os::raw::c_void;

		//need just the reference to topmost one,
		//but other parser have to be kept alive as well
		//preparsers keeps them alive
		let mut preparsers: Vec<parser_ptr> = Vec::new();
		let $top = mpc_new(c_str!(stringify!($top)));
		$(
			let $p = mpc_new(c_str!(stringify!($p)));
		)+

		mpca_lang(
			mpca_lang_type::MPCA_LANG_DEFAULT,
			c_str!($grammar)
			$(, $p)+,
			$top,
			0 as *mut c_void
		);

		preparsers.push($top);
		$(
			preparsers.push($p);
		)+
		preparsers
	}}};
}

/// Runs a parser prepared with `parser!`
#[macro_export]
macro_rules! run_parser
{
	(preparsers: $preparsers:ident
	 filename: {$filename:expr}
	 input: {$input:expr}) =>
	{{
		use mpc::glue;

		glue::parse(
			c_str!($filename),
			c_str!($input),
			$preparsers[0]
		)
	}};
	(preparsers: $preparsers:ident
	 input: {$input:expr}) =>
	{{
		use mpc::glue;

		glue::parse(
			c_str!("<input>"),
			c_str!($input),
			$preparsers[0]
		)
	}};
	(preparsers: $preparsers:ident
	 filename: {$filename:expr}) =>
	{{
		use mpc::glue;
		use std::fs::File;

		let mut input = String::new();
		if let Ok(mut ay) = File::open($filename)
		{
			ay.read_to_string(&mut input);
		};
		if input == String::new()
			{panic!();}
		glue::parse(
			c_str!($filename),
			c_str!(input),
			$preparsers[0]
		)
	}}
}
