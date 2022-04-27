use anyhow::Result;
use tonic_build;

use prost_build::Config;

fn main()  {
    prost_build::Config::new()
        .out_dir("/Users/andy/CLionProjects/untitled7/src")
        .compile_protos(&["/Users/andy/CLionProjects/untitled7/src/items.proto"], &["/Users/andy/CLionProjects/untitled7/src/"])
        .unwrap();


    //tonic_build::configure().out_dir("/Users/andy/CLionProjects/untitled7/src").compile(&["/Users/andy/CLionProjects/untitled7/src/items.proto"], &["/Users/andy/CLionProjects/untitled7/src/"]).unwrap();
//Ok(())
}