use std::{io::Write, net::{Ipv4Addr, SocketAddr, UdpSocket}};
fn main(){
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED,9090)).unwrap();
    let local_addr = sock.local_addr().unwrap();
    println!("224,0,2,1 is mulitcast {}",Ipv4Addr::from([224,0,2,1]).is_multicast());
    println!("local {local_addr}");
    sock.set_broadcast(true).unwrap();
    sock.set_multicast_loop_v4(true).unwrap();
    sock.join_multicast_v4(&Ipv4Addr::from([224,0,2,1]), &Ipv4Addr::UNSPECIFIED).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(5000));
    for i in 0..100{
        sock.send_to(b"hello multicast", SocketAddr::from(([224,0,2,1],9090))).unwrap();
        //sock.send_to(b"hello broadcast", SocketAddr::from(([255,255,255,255],9090))).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        print!("send {} times\r",i+1);
        std::io::stdout().flush().unwrap();
    }
    println!();
    println!("send over");
    sock.send_to(b"hello dedicated", SocketAddr::from(([192,168,1,158],9090))).unwrap();
}