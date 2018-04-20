use std::net;

fn main() {
    let socket = net::UdpSocket::bind("127.0.0.1:13662").expect("Can't bind as address");
    println!("{}", socket.local_addr().unwrap());
    let mut buffer: [u8; 16] = [0; 16];
    //socket.set_write_timeout(Some(std::time::Duration::new(1,0)));
    //println!("Write timeout: {:?}",socket.write_timeout().unwrap());

    loop {
        let begin_signal = socket.recv_from(&mut buffer).unwrap();
        if b"Begin!" != &buffer[0..begin_signal.0] {
            println!("Not expected packet: {:?}, from {}", &buffer[0..begin_signal.0], begin_signal.1);
            continue;
        }
        println!("Begin Receiving");
        socket.set_read_timeout(Some(std::time::Duration::from_millis(60))).expect("Set read timeout failed");
        for i in 0..10001 {
            let output = socket.recv_from(&mut buffer).expect("Connection timed out: ");
            let actual = ((buffer[0] as u16) << 8) + buffer[1] as u16;
            if i != actual {
                panic!("Dropped Packet");
            }
            println!("{:?}", &buffer[0..output.0]);
        }
        println!("Complete");
        socket.set_read_timeout(None).expect("Set read timeout failed");
    }
}
