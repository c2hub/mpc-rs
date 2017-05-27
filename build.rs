extern crate gcc;

fn main()
{
	gcc::compile_library("libmpc.a", &["src/mpc.c", "src/glue.c"]);
}
