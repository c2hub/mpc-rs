#[macro_export]
macro_rules! parser
{
	($grammar:expr, $input:expr, $filename:expr, $top:ident, $($p:ident)+) =>
	{{ unsafe {
		use std::ffi::CString;
		use std::os::raw::c_void;
		let $top = mpc_new(CString::new(stringify!($top)).unwrap().as_ptr());
		$
		(
			let $p = mpc_new(CString::new(stringify!($p)).unwrap().as_ptr());
		)+

		mpca_lang(
			mpca_lang_type::MPCA_LANG_DEFAULT, 
			CString::new($grammar).unwrap().as_ptr()
			$(, $p)+, 
			$top, 
			0 as *mut c_void
		);

		let res = glue_parse(
			CString::new($filename).unwrap().as_ptr(),
			CString::new($input).unwrap().as_ptr(),
			$top,
		);

		match res.res
		{
			res_t::ok => Ok(res.ok),
			res_t::err => Err(res.err)
		}
	}}}
}
