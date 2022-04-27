


extern crate libc;

use libc::c_char;
use std::ffi::CString;

fn main() {
    let c_str_1 = CString::new("hello").unwrap(); // from a &str, creates a new allocation
    let c_str_2 = CString::new(b"world" as &[u8]).unwrap(); // from a &[u8], creates a new allocation
    let data: Vec<u8> = b"12345678".to_vec(); // from a Vec<u8>, consumes it
    let c_str_3 = CString::new(data).unwrap();

    // and now you can obtain a pointer to a valid zero-terminated string
    // make sure you don't use it after c_str_2 is dropped
    let c_ptr: *const c_char = c_str_2.as_ptr();

    // the following will print an error message because the source data
    // contains zero bytes
    let data: Vec<u8> = vec![1, 2, 3, 7, 4, 5, 9, 6];
    match CString::new(data) {
        Ok(c_str_4) => println!("Got a C string: {:p}", c_str_4.as_ptr()),
        Err(e) => println!("Error getting a C string: {}", e),
    }
}
