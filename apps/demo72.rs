

use std::os::raw::{c_char, c_float, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CStudent {
    pub num: ::std::os::raw::c_int,
    pub total: ::std::os::raw::c_int,
    pub name: [::std::os::raw::c_char; 20usize],
    pub scores: [f32; 3usize],
}

extern "C" {
    pub fn print_data(stu: *mut CStudent);
}
extern "C" {
    pub fn fill_data(stu: *mut CStudent);
}

// Default constructor
impl Default for CStudent {
    fn default() -> Self {
        CStudent {
            num: 0 as c_int,
            total: 0 as c_int,
            name: [0 as c_char; 20],
            scores: [0.0 as c_float; 3],
        }
    }
}





fn main() {
    // Initialization of allocated memory
    let new_stu: CStudent = Default::default();
    println!("rust side print new_stu: {:?}", new_stu);
    let box_new_stu = Box::new(new_stu);
    let p_stu = Box::into_raw(box_new_stu);

    unsafe {
        fill_data(p_stu);
        print_data(p_stu);
        //Box::from_raw(p_stu);
        println!("rust side print Bob: {:?}", Box::from_raw(p_stu));
    }
}
