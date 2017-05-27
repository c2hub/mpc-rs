#[macro_export]
macro_rules! c_str /* C to String */
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
macro_rules! parser
{
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
}
