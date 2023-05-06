use socket2::{Domain, Protocol, Socket, Type};
use std::io::Read;

fn open_socket() -> Socket {
    // Create the ICMP socket
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)).unwrap();

    socket
}

fn main() {
    let mut socket = open_socket();

    let mut buf = [0u8; 1024];

    loop {
        let packet_len = socket.read(&mut buf).unwrap();

        println!("RECIEVED {:?}", &buf[..packet_len]);
    }
}
