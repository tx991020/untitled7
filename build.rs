fn main() {

    println!("cargo:rustc-link-search=native={}", "/Users/andy/CLionProjects/untitled7/clib");
    println!("cargo:rustc-link-lib=static=cfoo");
    // println!("cargo:rustc-link-lib=static=factorial");

    // cc::Build::new()
    //     .file("csrc/cfoo.c")
    //     .compile("cfoo");
}