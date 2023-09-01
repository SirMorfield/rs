extern crate cc;
use glob::glob;

fn main() {
	let paths : Vec<String> = glob("./src_c/**/*.c")
		.unwrap()
		.map(|path| path.unwrap().display().to_string())
		.collect();

	for path in &paths {
		println!("cargo:rerun-if-changed={}", path);
	}

	cc::Build::new()
		.files(paths)
		.flag("-Wall")
		.flag("-Wextra")
		.flag("-Werror")
		.flag("-Wno-unused-parameter")
		.compile("main");
}