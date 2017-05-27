#[macro_export]
macro_rules! c_str
{
	($str:expr) => 
	{{
		use std::ffi::CString;
		CString::new($str).unwrap().as_ptr()
	}}
}

#[macro_export]
macro_rules! str_c
{
	($ptr:expr) =>
	{{
		use std::ffi::CStr;
		CStr::from_ptr($ptr).to_string_lossy().into_owned()
	}}
}

#[macro_export]
macro_rules! dfs
{
	($ptr:expr, $mem:ident) =>
	{{
		str_c!((*$ptr).$mem)
	}}
}

#[macro_export]
macro_rules! parser
{
	($grammar:expr, $input:expr, $filename:expr, $top:ident, $($p:ident)+) =>
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
	}}}
}
