#[macro_export]
macro_rules! c_str /* C-ish String */
{
	($str:expr) =>
	{{
		use std::ffi::CString;
		CString::new($str).unwrap().as_ptr()
	}}
}

#[macro_export]
macro_rules! str_c /* String from C */
{
	($ptr:expr) =>
	{{
		use std::ffi::CStr;
		CStr::from_ptr($ptr).to_string_lossy().into_owned()
	}}
}

#[macro_export]
macro_rules! dfs /* dereferenced's member to String */
{
	($ptr:expr, $mem:ident) =>
	{{
		str_c!((*$ptr).$mem)
	}}
}

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

#[macro_export]
macro_rules! g_string /* couldn't help it */
{
	($($arg:expr)+) =>
	{{
		concat!( $( $arg, )+ )
	}}
}

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
		use glue;
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
		use glue;
		use std::os::raw::c_void;
		use std::fs::File;

		let mut input = String::new();
		if let Some(thing) = File::open($filename)
		{
			thing.read_to_string(&input);
		} else { None } /* expecting user to check if the
		                 * file's readable beforehand
		                 */
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
	 filename: {$filename:expr}
	 input: {$input:expr}
	 main: $top:ident
	 parsers: $($p:ident)+) =>
	{{ unsafe {
		use glue;
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
		use glue;
		use std::os::raw::c_void;
		use std::fs::File;

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

#[macro_export]
macro_rules! run_parser
{
	(preparsers: $preparsers:ident
	 filename: {$filename:expr}
	 input: {$input:expr}) =>
	{{
		use glue;

		glue::parse(
			c_str!($filename),
			c_str!($input),
			$preparsers[0]
		);
	}};
	(preparsers: $preparsers:ident
	 input: {$input:expr}) =>
	{{
		use glue;

		glue::parse(
			c_str!("<input>"),
			c_str!($input),
			$preparsers[0]
		);
	}};
	(preparsers: $preparsers:ident
	 filename: {$filename:expr}) =>
	{{
		use glue;
		use std::fs::File;

		let mut input = String::new();
		if let Some(thing) = File::open($filename)
		{
			thing.read_to_string(&input);
		} else { None } /* expecting user to check if the
		                 * file's readable beforehand
		                 */
		glue::parse(
			c_str!($filename),
			c_str!(input),
			$preparsers[0]
		);
	}}
}
