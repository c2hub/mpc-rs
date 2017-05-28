extern crate gcc;

fn main()
{
	gcc::Config::new()
		.file("src/mpc.c")
		.file("src/glue.c")
		.flag("-std=gnu99")
		.compile("libmpc.a");
}
