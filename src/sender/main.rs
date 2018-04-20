extern crate rand;
use std::net;
use rand::Rng;

fn main() {
    let socket = net::UdpSocket::bind("127.0.0.1:13661").expect("Can't bind as address");

    socket.send_to(b"Begin!", "127.0.0.1:13662".parse::<net::SocketAddr>().unwrap()).expect("Sending Failed");
    for i in 0..10001u16 {
        socket.send_to(&[((i >> 8) & 0xff) as u8, (i & 0xff) as u8], "127.0.0.1:13662".parse::<net::SocketAddr>().unwrap()).expect("Sending Failed");
        println!("{}",i);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
