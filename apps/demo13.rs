



fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    // function (actually a pointer) is Copy
    is_copy::<fn()>();

    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // immutable reference is Copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    is_copy::<&str>();
    // array/tuple with values which is Copy is Copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

// fn types_not_impl_copy_trait() {
//     // unsized or dynamic sized type is not Copy
//     is_copy::<str>();
//     is_copy::<[u8]>();
//     is_copy::<Vec<u8>>();
//     is_copy::<String>();
//
//     // mutable reference is not Copy
//     is_copy::<&mut String>();
//
//     // array / tuple with values that not Copy is not Copy
//     is_copy::<[Vec<u8>; 4]>();
//     is_copy::<(String, u32)>();
// }

fn main1() {
    types_impl_copy_trait();
    // types_not_impl_copy_trait();
}
fn sum(data:&Vec<u32>) -> u32 { // 值的地址会改变么？引用的地址会改变么？
println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}
fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!( "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", &data, data1, &&data, &data1 );
    println!("sum of data1: {}", sum(data1));


}