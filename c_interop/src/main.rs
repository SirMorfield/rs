use std::os::raw::c_char;
use std::os::raw::c_int;
mod helpers;
use helpers::vector_to_cstring_vector;

extern {
	fn mainj(
        argc: c_int,
        argv: *mut *mut c_char,
    ) -> c_int;

}

fn main() {
	let result: i32;
    unsafe {
		let argv = std::env::args().collect::<Vec<_>>();

        result = mainj(0, vector_to_cstring_vector(argv));
    }
    println!("The result from C is: {}", result);
}