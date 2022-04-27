

extern crate libc;

use libc::c_char;
use std::ops::Deref;
use std::ffi::CStr;

extern "C" {
    fn hello() -> *const c_char;
    fn goodbye(s: *const c_char);
}

struct Greeting {
    message: *const c_char,
}

impl Drop for Greeting {
    fn drop(&mut self) {
        unsafe {
            goodbye(self.message);
        }
    }
}

impl Greeting {
    fn new() -> Greeting {
        Greeting { message: unsafe { hello() } }
    }
}

impl Deref for Greeting {
    type Target = str;

    fn deref<'a>(&'a self) -> &'a str {
        let c_str = unsafe { CStr::from_ptr(self.message) };
        c_str.to_str().unwrap()
    }
}

fn main() {}