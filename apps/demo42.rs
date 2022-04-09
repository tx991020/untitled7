use std::net::{SocketAddr, ToSocketAddrs};
use std::string::String;
use anyhow::bail;
use sled::{Config, Result};

pub fn to_socket_addr(host: &str) -> ResultType<SocketAddr> {
    let addrs: Vec<SocketAddr> = host.to_socket_addrs()?.collect();
    if addrs.is_empty() {
        bail!("Failed to solve {}", host);
    }
    Ok(addrs[0])
}


fn main()->Result<()> {
    // let config = Config::new().temporary(true);
    //
    // let db = config.open()?;
    //
    // let k = "aaaaa";
    // let v = "wowo";
    //
    // // set and get
    // db.insert(k, v)?;
    // let vec = db.get(k)?.unwrap();
    // println!("{}",String::from_utf8_lossy(vec.as_ref()) );
    let addr = to_socket_addr("0.0.0.0:0");


    Ok(())

}