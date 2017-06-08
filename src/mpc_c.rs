//! Contains `mpc` function definitions
#![allow(dead_code)]
#![allow(improper_ctypes)]

use mpc_c_types::*;
use std::os::raw::{c_char, c_void};

extern
{
	/*
	** Errors
	** TODO mpc_err_print_to
	*/
	/// delete a `*mut mpc_err_t`, should be used instead of free()
	pub fn mpc_err_delete(e: *mut mpc_err_t);
	/// get an error string of error `e`
	pub fn mpc_err_string(e: *const mpc_err_t) -> *const c_char;
	/// print a error to `stdout`
	pub fn mpc_err_print(e: *const mpc_err_t);

	/*
	** Parsing
	** TODO mpc_parse_file & mpc_parse_pipe
	*/
	/// run a parser on some string
	pub fn mpc_parse(filename: *const c_char, string: *const c_char, p: *mut mpc_parser_t, r: *const mpc_result_t) -> i32;
	/// run a parser on `length` characters of some string
	pub fn mpc_nparse(filename: *const c_char, string: *const c_char, length: u32, p: *mut mpc_parser_t, r: *const mpc_result_t) -> i32;
	/// parse contents of a file `filename`
	pub fn mpc_parse_contents(filename: *const c_char, p: *mut mpc_parser_t, r: *const mpc_result_t);

	/*
	** Building a Parser
	*/
	/// construct a new parser called `name`
	pub fn mpc_new(name: *const c_char) -> *mut mpc_parser_t;
	/// make a copy of the parser `a`
	pub fn mpc_copy(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	/// assign contents of parser `a` to parser `p`, delete `a`
	pub fn mpc_define(p: *mut mpc_parser_t, a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	/// undefine a parser. use this before deleting a parser
	pub fn mpc_undefine(p: *mut mpc_parser_t) -> *mut mpc_parser_t;
	/// automatically undefine and delete parsers
	pub fn mpc_cleanup(n: i32, ...);

	/*
	** Basic Parsers
	*/
	/// matches any individual character
	pub fn mpc_any() -> *mut mpc_parser_t;
	/// matches a single character `c`
	pub fn mpc_char(c: c_char) -> *mut mpc_parser_t;
	/// matches a character in the range from `s` to `e`
	pub fn mpc_range(s: c_char, e: c_char) -> *mut mpc_parser_t;
	/// matches one of characters `s`
	pub fn mpc_oneof(s: *const c_char) -> *mut mpc_parser_t;
	/// matches none of characters `s`
	pub fn mpc_noneof(s: *const c_char) -> *mut mpc_parser_t;
	/// matches a character that satisfies function `f`
	pub fn mpc_satisfy(f: extern fn(c: c_char) -> i32) -> *mut mpc_parser_t;
	/// matches a string `s`
	pub fn mpc_string(s: *const c_char) -> *mut mpc_parser_t;

	/*
	** Other Parsers
	** TODO mpc_failf
	*/
	/// consumes no input, always successful, returns a nullptr
	pub fn mpc_pass() -> *mut mpc_parser_t;
	/// consumes no input, always fails with message `m`
	pub fn mpc_fail(m: *const c_char) -> *mut mpc_parser_t;
	/// consumes no input, always fails with formated message `fmt`
	pub fn mpc_failf(fmt: *const c_char, ...) -> *mut mpc_parser_t;
	/// consumes no input, always successful, returns the result of function `f`
	pub fn mpc_lift(f: mpc_ctor_t) -> *mut mpc_parser_t;
	/// consumes no input, always sucessful, returns value `x`
	pub fn mpc_lift_val(x: *mut mpc_val_t) -> *mut mpc_parser_t;
	/// Consumes no input. Successful when function `f` returns true. Always returns `NULL`.
	///
	/// Function `f` is a anchor function. It takes as input the last character parsed,
	/// and the next character in the input, and returns success or failure. This function
	/// can be set by the user to ensure some condition is met. For example to test
	/// that the input is at a boundary between words and non-words.
	///
	/// At the start of the input the first argument is set to '\0'.
	/// At the end of the input the second argument is set to '\0'.
	pub fn mpc_anchor(f: extern fn(c1: c_char, c2: c_char) -> i32) -> *mut mpc_parser_t;
	/// consumes no input, always successful, returns copy of the parser state as `*mut mpc_parser_t`.
	/// the state is newly allocated and needs to be freed
	pub fn mpc_state() -> *mut mpc_parser_t;

	/*
	** Combinator Parsers
	*/
	/// returns a parser that runs `a` and on success returns the result of `a`, on failure returns `e`
	pub fn mpc_expect(a: *mut mpc_parser_t, e: *const c_char) -> *mut mpc_parser_t;
	/// returns a parser that runs `a` and on success returns the result of `a`, on failure formatted message `fmt`
	pub fn mpc_expectf(a: *mut mpc_parser_t, fmt: *const c_char, ...) -> *mut mpc_parser_t;
	/// returns a parser that applies function `f` to the result of parser `a`
	pub fn mpc_apply(a: *mut mpc_parser_t, f: mpc_apply_t) -> *mut mpc_parser_t;
	/// returns a parser that applies function `f` to the result of parser `a`, taking extra input `x`
	pub fn mpc_apply_to(a: *mut mpc_parser_t, f: mpc_apply_to_t, x: *const c_void) -> *mut mpc_parser_t;

	/// returns a parser that, if `a` succeeds, fails and consumes no input, if `a` fails, it succeeds and consumes no input.
	/// destructor `da` is to destroy the result of `a` on success
	pub fn mpc_not(a: *mut mpc_parser_t, da: mpc_dtor_t) -> *mut mpc_parser_t;
	/// returns a parser that, if `a` succeeds, fails and consumes no input, if `a` fails, it succeeds and consumes no input
	/// and returns result of lift function `lf`. destructor `da` is to destroy the result of `a` on success
	pub fn mpc_not_lift(a: *mut mpc_parser_t, da: mpc_dtor_t, lf: mpc_ctor_t) -> *mut mpc_parser_t;
	/// returns a parser that runs `a`, if it succeeds, returns the result of `a`, if `a` doesn't succeed, it succeeds, but
	/// returns a nullptr
	pub fn mpc_maybe(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	/// returns a parser that runs `a`, if it succeeds, returns the result of `a`, if `a` doesn't succeed, it succeeds, but
	/// returns the result of lift function `lf`
	pub fn mpc_maybe_lift(a: *mut mpc_parser_t, lf: mpc_ctor_t) -> *mut mpc_parser_t;

	/// runs parser `a` zero or more times, until it fails. Results are combined using fold function `f`
	pub fn mpc_many(f: mpc_fold_t, a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	/// runs parser `a` one or more times, until it fails. Results are combined using fold function `f`
	pub fn mpc_many1(f: mpc_fold_t, a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	/// runs parser `a` exactly `n` times, on failure, partial results are destroyed with `da`, on success
	/// results are combined using fold function `f`
	pub fn mpc_count(n: i32, f: mpc_fold_t, a: *mut mpc_parser_t, lf: mpc_ctor_t) -> *mut mpc_parser_t;

	/// attempts to run `n` parsers in sequence, returning the first one that succeeds
	pub fn mpc_or(n: i32, ...) -> *mut mpc_parser_t;
	/// attempts to run `n` parsers in sequence, combining results with fold function `f`
	pub fn mpc_and(n: i32, f: mpc_fold_t, ...) -> *mut mpc_parser_t;

	/// runs a parser with backtracking disabled
	pub fn mpc_predictive(a: *mut mpc_parser_t) -> *mut mpc_parser_t;

	/*
	** Common Parsers
	*/
	/// matches only the start of input
	pub fn mpc_eoi() -> *mut mpc_parser_t;
	/// matches only the end of input
	pub fn mpc_soi() -> *mut mpc_parser_t;

	/// matches only the boundary between words
	pub fn mpc_boundary() -> *mut mpc_parser_t;

	/// matches any whitespace character
	pub fn mpc_whitespace() -> *mut mpc_parser_t;
	/// matches zero or more whitespace characters
	pub fn mpc_whitespaces() -> *mut mpc_parser_t;
	/// matches whitespace and frees result
	pub fn mpc_blank() -> *mut mpc_parser_t;

	/// matches a newline
	pub fn mpc_newline() -> *mut mpc_parser_t;
	/// matches a tab
	pub fn mpc_tab() -> *mut mpc_parser_t;
	/// matches an escape sequence
	pub fn mpc_escape() -> *mut mpc_parser_t;
	/// matches a single digit
	pub fn mpc_digit() -> *mut mpc_parser_t;
	/// matches a single hex digit (0-9, A-F
	pub fn mpc_hexdigit() -> *mut mpc_parser_t;
	/// matches a single octal digit
	pub fn mpc_octdigit() -> *mut mpc_parser_t;
	/// matches one or more digits
	pub fn mpc_digits() -> *mut mpc_parser_t;
	/// matches one or more hex digits
	pub fn mpc_hexdigits() -> *mut mpc_parser_t;
	/// matches one or more octal digits
	pub fn mpc_octdigits() -> *mut mpc_parser_t;

	/// matches any lowercase character
	pub fn mpc_lower() -> *mut mpc_parser_t;
	/// matches any upper case character
	pub fn mpc_upper() -> *mut mpc_parser_t;
	/// matches any alphabet character
	pub fn mpc_alpha() -> *mut mpc_parser_t;
	/// matches an underscore
	pub fn mpc_underscore() -> *mut mpc_parser_t;
	/// matches a any alphabet character, an underscore or a digit
	pub fn mpc_alphanum() -> *mut mpc_parser_t;

	/// matches digits and returns `*mut int`
	pub fn mpc_int() -> *mut mpc_parser_t;
	/// matches hex digits and returns `*mut i32`
	pub fn mpc_hex() -> *mut mpc_parser_t;
	/// matches octal digits and returns `*mut i32`
	pub fn mpc_oct() -> *mut mpc_parser_t;
	/// matches any digits and returns `*mut i32`
	pub fn mpc_number() -> *mut mpc_parser_t;

	/// matches floating point numbers as a string
	pub fn mpc_real() -> *mut mpc_parser_t;
	/// matches floating point numbers as a `*mut f32`
	pub fn mpc_float() -> *mut mpc_parser_t;

	/// matches a char literal
	pub fn mpc_char_lit() -> *mut mpc_parser_t;
	/// matches a string literal
	pub fn mpc_string_lit() -> *mut mpc_parser_t;
	/// matches a regex literal
	pub fn mpc_regex_lit() -> *mut mpc_parser_t;

	/// matches a valid C identifier
	pub fn mpc_ident() -> *mut mpc_parser_t;

	/*
	** Useful Parsers
	*/
	/// matches the start of input followed by `a`
	pub fn mpc_startwith(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	/// matches the end of input followed by `a`
	pub fn mpc_endwith(a: *mut mpc_parser_t, da: mpc_dtor_t) -> *mut mpc_parser_t;
	/// matches input from start to end
	pub fn mpc_whole(a: *mut mpc_parser_t, da: mpc_dtor_t) -> *mut mpc_parser_t;

	/// matches input starting
	pub fn mpc_stripl(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpc_stripr(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpc_strip(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpc_tok(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpc_sym(s: *const c_char) -> *mut mpc_parser_t;
	pub fn mpc_total(a: *mut mpc_parser_t, da: mpc_dtor_t) -> *mut mpc_parser_t;

	pub fn mpc_between(a: *mut mpc_parser_t, ad: mpc_dtor_t, o: *const c_char, c: *const c_char) -> *mut mpc_parser_t;
	pub fn mpc_parens(a: *mut mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;
	pub fn mpc_braces(a: *mut mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;
	pub fn mpc_brackets(a: *mut mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;
	pub fn mpc_squares(a: *mut mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;

	pub fn mpc_tok_between(a: *mut mpc_parser_t, ad: mpc_dtor_t, o: *const c_char, c: *const c_char) -> *mut mpc_parser_t;
	pub fn mpc_tok_parens(a: *mut mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;
	pub fn mpc_tok_braces(a: *mut mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;
	pub fn mpc_tok_brackets(a: *mut mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;
	pub fn mpc_tok_squares(a: mpc_parser_t, ad: mpc_dtor_t) -> *mut mpc_parser_t;

	/*
	** Common Function Parameters
	*/
	pub fn mpcf_dtor_null(x: *mut mpc_val_t);

	pub fn mpcf_ctor_null() -> *mut mpc_val_t;
	pub fn mpcf_ctor_str() -> *mut mpc_val_t;

	pub fn mpcf_free(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_int(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_hex(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_oct(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_float(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_strtriml(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_strtrimr(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_strtrim(x: *mut mpc_val_t) -> *mut mpc_val_t;

	pub fn mpcf_escape(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_escape_regex(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_escape_string_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_escape_char_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;

	pub fn mpcf_unescape(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_unescape_regex(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_unescape_string_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_unescape_char_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;

	pub fn mpcf_null(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_fst(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_snd(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_trd(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;

	pub fn mpcf_fst_free(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_snd_free(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_trd_free(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;

	pub fn mpcf_strfold(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_maths(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;

	/*
	** Regular Expession Parsers
	*/
	pub fn mpc_re(re: *const c_char) -> *mut mpc_parser_t;

	/*
	** AST
	** TODO mpc_ast_print_to,
	**      mpca_lang_file, mpca_lang_pipe,
	*/
	pub fn mpc_ast_new(tag: *const c_char, contents: *const c_char) -> *mut mpc_ast_t;
	pub fn mpc_ast_build(n: i32, tag: *const char, ...) -> *mut mpc_ast_t;
	pub fn mpc_ast_add_root(a: *mut mpc_ast_t) -> *mut mpc_ast_t;
	pub fn mpc_ast_add_child(r: *mut mpc_ast_t, a: *mut mpc_ast_t) -> *mut mpc_ast_t;
	pub fn mpc_ast_add_tag(a: *mut mpc_ast_t, t: *const c_char) -> *mut mpc_ast_t;
	pub fn mpc_ast_add_root_tag(a: *mut mpc_ast_t, t: *const c_char) -> *mut mpc_ast_t;
	pub fn mpc_ast_tag(a: *mut mpc_ast_t, t: *const c_char) -> *mut mpc_ast_t;
	pub fn mpc_ast_state(a: *mut mpc_ast_t, s: mpc_state_t) -> *mut mpc_ast_t;

	pub fn mpc_ast_delete(a: *mut mpc_ast_t);
	pub fn mpc_ast_print(a: *mut mpc_ast_t);

	pub fn mpc_ast_get_index(ast: *mut mpc_ast_t, tag: *const c_char);
	pub fn mpc_ast_get_index_lb(ast: *mut mpc_ast_t, tag: *const c_char, lb: i32);
	pub fn mpc_ast_get_child(ast: *mut mpc_ast_t, tag: *const c_char);
	pub fn mpc_ast_get_child_lb(ast: *mut mpc_ast_t, tag: *const c_char, lb: i32);

	pub fn mpc_ast_traverse_start(ast: *mut mpc_ast_t, order: mpc_ast_trav_order_t) -> *mut mpc_ast_trav_t;
	pub fn mpc_ast_traverse_next(trav: *mut *mut mpc_ast_trav_t) -> *mut mpc_ast_t;
	pub fn mpc_ast_traverse_free(trav: *mut *mut mpc_ast_trav_t);

	pub fn mpcf_fold_ast(n: i32, _as: *mut *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_str_ast(a: *mut mpc_val_t) -> *mut mpc_val_t;
	pub fn mpcf_state_ast(n: i32, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;

	pub fn mpca_tag(a: *mut mpc_parser_t, t: *const c_char) -> *mut mpc_parser_t;
	pub fn mpca_add_tag(a: *mut mpc_parser_t, t: *const c_char) -> *mut mpc_parser_t;
	pub fn mpca_root(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpca_state(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpca_total(a: *mut mpc_parser_t) -> *mut mpc_parser_t;

	pub fn mpca_not(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpca_maybe(a: *mut mpc_parser_t) -> *mut mpc_parser_t;

	pub fn mpca_or(n: i32, ...) -> *mut mpc_parser_t;
	pub fn mpca_and(n: i32, ...) -> *mut mpc_parser_t;

	pub fn mpca_many(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpca_many1(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
	pub fn mpca_count(n: i32, a: *mut mpc_parser_t) -> *mut mpc_parser_t;

	pub fn mpca_grammar(flags: mpca_lang_type, grammar: *const c_char, ...) -> *mut mpc_parser_t;
	pub fn mpca_lang(flags: mpca_lang_type, grammar: *const c_char, ...) -> *mut mpc_parser_t;
	pub fn mpca_lang_contents(flags: mpca_lang_type, filename: *const c_char, ...) -> *mut mpc_parser_t;

	/*
	** Misc
	*/
	pub fn mpc_print(p: *mut mpc_parser_t);
	pub fn mpc_optimise(p: *mut mpc_parser_t);
	pub fn mpc_stats(p: *mut mpc_parser_t);

	pub fn mpc_test_pass(
		p: *mut mpc_parser_t,
		s: *const c_char,
		d: *const c_void,
		tester: extern fn(a1: *const c_void, a2: *const c_void) -> i32,
		destructor: mpc_dtor_t,
		printer: extern fn(p: *const c_void)
	) -> i32;

	pub fn mpc_test_fail(
		p: *mut mpc_parser_t,
		s: *const c_char,
		d: *const c_void,
		tester: extern fn(a1: *const c_void, a2: *const c_void) -> i32,
		destructor: mpc_dtor_t,
		printer: extern fn(p: *const c_void)
	) -> i32;
}
