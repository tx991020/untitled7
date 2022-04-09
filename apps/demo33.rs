

use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::{thread, time};
use anyhow::Result;
static MULTI_CAST_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 42, 98);
pub fn listen() ->Result<()>{
    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 50692);
    let bind_addr = Ipv4Addr::new(0, 0, 0, 0);
    let socket = UdpSocket::bind(socket_address)?;
    println!("Listening on: {}", socket.local_addr().unwrap());
    socket.join_multicast_v4(&MULTI_CAST_ADDR, &bind_addr)?;
    Ok(())
}
pub fn cast() -> Result<()> {

    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 50692);
    let socket = UdpSocket::bind(socket_address)?;
    println!("{}",444 );
    socket.connect(SocketAddrV4::new(MULTI_CAST_ADDR, 50692))?;
    // Don't send messages to yourself.
    // In this case self discovery is for human developers, not machines.
    println!("{}",3333 );
    socket.set_multicast_loop_v4(false)?;
    println!("{}",2222 );
    let data = String::from("{\"username\": \"test\"}");
    loop {
        socket.send(data.as_bytes())?;
        thread::sleep(time::Duration::from_secs(2));
    }
    println!("{}",1111 );
    Ok(())
}
fn main() {
    // thread::spawn(||{
    //     listen();
    // });
    cast();
}