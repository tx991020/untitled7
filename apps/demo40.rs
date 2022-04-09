use std::env;
use std::path::{Path, PathBuf};

fn main() {

    let includes = find_package("libvpx");
    println!("{:?}",includes );

    gen_vpx()
    // let src_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    // let src_dir = Path::new(&src_dir);
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let out_dir = Path::new(&out_dir);
    //
    // let ffi_header = src_dir.join("vpx_ffi.h");
    // println!("rerun-if-changed={}", ffi_header.display());
    // for dir in &includes {
    //     println!("rerun-if-changed={}", dir.display());
    // }
    //
    // let ffi_rs = out_dir.join("vpx_ffi.rs");
    // let exact_file = src_dir.join("generated").join("vpx_ffi.rs");
}


fn find_package(name: &str) -> Vec<PathBuf> {
    let vcpkg_root = std::env::var("VCPKG_ROOT").unwrap();
    let mut path: PathBuf = vcpkg_root.into();
    let mut target_arch = "x86_64".to_string();
    if target_arch == "x86_64" {
        target_arch = "x64".to_owned();
    } else if target_arch == "aarch64" {
        target_arch = "arm64".to_owned();
    }
    let target_os = "macos";
    let mut target = if target_os == "macos" {
        "x64-osx".to_owned()
    } else if target_os == "windows" {
        "x64-windows-static".to_owned()
    } else {
        format!("{}-{}", target_arch, target_os)
    };
    if target_arch == "x86" {
        target = target.replace("x64", "x86");
    }
    println!("cargo:info={}", target);
    path.push("installed");
    path.push(target);
    let lib = name.trim_start_matches("lib").to_string();
    println!("{}", format!("cargo:rustc-link-lib=static={}", lib));
    println!(
        "{}",
        format!(
            "cargo:rustc-link-search={}",
            path.join("lib").to_str().unwrap()
        )
    );
    let include = path.join("include");
    println!("{}", format!("cargo:include={}", include.to_str().unwrap()));

    vec![include]
}


fn gen_vpx() {

    let includes = find_package("libvpx");
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    println!("zzz {:?}",src_dir );
    let src_dir = Path::new(&src_dir);
    let out_dir = Path::new("/Users/andy/CLionProjects/untitled7/apps/");
    println!("xxxxx  {:?},{:?},{:?}", includes,src_dir,out_dir);
    let ffi_header = src_dir.join("vpx_ffi.h");
    println!("rerun-if-changed={}", ffi_header.display());
    for dir in &includes {
        println!("rerun-if-changed={}", dir.display());
    }

    let ffi_rs = out_dir.join("vpx_ffi.rs");
    let exact_file = src_dir.join("generated").join("vpx_ffi.rs");
    println!("jjjjj {:?},{:?}",ffi_rs,exact_file );
}