

extern "C" {
    pub fn Factorial(n: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}


fn main() {
    let i = unsafe {
        let i = Factorial(5);
        i
    };
    print!("{}",i);
}