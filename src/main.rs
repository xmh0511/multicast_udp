use std::net::{Ipv4Addr, UdpSocket};
fn main() {
    const MULTI_CAST_ADDR:Ipv4Addr = Ipv4Addr::new(224,0,2,1);
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED,9090)).unwrap();
    println!("{MULTI_CAST_ADDR} is mulitcast {}",MULTI_CAST_ADDR.is_multicast());
    sock.set_broadcast(true).unwrap();
    sock.set_multicast_loop_v4(true).unwrap();
    sock.join_multicast_v4(&MULTI_CAST_ADDR, &Ipv4Addr::UNSPECIFIED).unwrap();
    let mut buf = [0u8;u16::MAX as _];
    while let Ok((size,addr)) = sock.recv_from(& mut buf){
        println!("receive from {addr} content:{}",String::from_utf8_lossy(&buf[..size]));
    }
}
