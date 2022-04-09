use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("/Users/andy/CLionProjects/untitled7/src")
        .compile_protos(&["/Users/andy/CLionProjects/untitled7/src/items.proto"], &["/Users/andy/CLionProjects/untitled7/src/"])
        .unwrap();
    Ok(())
}