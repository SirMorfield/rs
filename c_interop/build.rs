extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/main.c")
        .compile("main");
}