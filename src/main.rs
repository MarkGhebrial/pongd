use socket2::{Domain, Protocol, Socket, Type};
use std::mem::MaybeUninit;

fn open_socket() -> Socket {
    // Create the ICMP socket
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)).unwrap();

    // let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    // socket.bind(&addr.into()).unwrap();

    socket
}

fn main() {
    let socket = open_socket();

    let mut buf: [MaybeUninit<u8>; 4000] = [MaybeUninit::new(0u8); 4000];

    let packet_len = socket.recv(&mut buf).unwrap();

    println!(
        "RECIEVED {:?}",
        buf[..packet_len]
            .iter()
            .map(|b| unsafe { b.assume_init() })
            .collect::<Vec<u8>>()
    );
}
