extern {
    fn print_hello_world() -> i32;
}

fn main() {
	let result: i32;
    unsafe {
        result = print_hello_world();
    }
    println!("The result from C is: {}", result);
}