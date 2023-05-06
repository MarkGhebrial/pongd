use socket2::{Domain, Protocol, Socket, Type};
use std::io::Read;
use std::io::Write;

use etherparse::{Icmpv4Slice, Icmpv4Type};
use etherparse::{Ipv4HeaderSlice, SlicedPacket};

fn open_socket() -> Socket {
    // Create the ICMP socket
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)).unwrap();

    socket
}

fn main() {
    let mut socket = open_socket();

    let mut buf = [0u8; 1024];

    loop {
        // Get a packet
        let packet_len = socket.read(&mut buf).unwrap();
        let packet = &buf[..packet_len];

        println!("RECIEVED {:?}", &buf[..packet_len]);

        // Parse the IP header
        let ipheader = Ipv4HeaderSlice::from_slice(packet).unwrap();

        // Parse the icmp header
        let icmp = Icmpv4Slice::from_slice(
            &packet[(ipheader.total_len() - ipheader.payload_len()) as usize..],
        )
        .unwrap();

        match icmp.icmp_type() {
            Icmpv4Type::EchoRequest(header) => {
                println!("THAT'S AN ECHO REQUEST!!! {:?}", header);
            }
            t => println!("{:?}", t),
        }
    }
}
