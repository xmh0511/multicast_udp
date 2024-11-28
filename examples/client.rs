use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
fn main(){
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED,9090)).unwrap();
    let local_addr = sock.local_addr().unwrap();
    println!("local {local_addr}");
    sock.join_multicast_v4(&Ipv4Addr::from([234,2,2,2]), &Ipv4Addr::UNSPECIFIED).unwrap();
    for _ in 0..30{
        sock.send_to(b"hello", SocketAddr::from(([234,2,2,2],9090))).unwrap();
    }
    println!("send over");
}