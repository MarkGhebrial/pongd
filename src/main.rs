use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4};

use std::fs::File;

use etherparse::Ipv4HeaderSlice;
use etherparse::{
    IcmpEchoHeader, Icmpv4Header, Icmpv4Slice, Icmpv4Type, Icmpv6Type, IpNumber, Ipv4Header,
};

fn open_socket() -> Socket {
    // Create the ICMP socket
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)).unwrap();

    socket
}

fn main() {
    let mut socket = open_socket();

    let mut buf = [0u8; 4096];

    loop {
        // Get a packet
        let packet_len = socket.read(&mut buf).unwrap();
        let packet = &buf[..packet_len];

        // Parse the IP header
        let incoming_ip_header = Ipv4HeaderSlice::from_slice(packet).unwrap();

        // Parse the icmp header
        let incoming_icmp_packet = Icmpv4Slice::from_slice(
            &packet[(incoming_ip_header.total_len() - incoming_ip_header.payload_len()) as usize..],
        )
        .unwrap();

        match incoming_icmp_packet.icmp_type() {
            Icmpv4Type::EchoRequest(header) => {
                println!("THAT'S AN ECHO REQUEST!!! {:?}", header);

                // Create the outgoing ICMP header
                let mut outgoing_icmp_header =
                    Icmpv4Header::new(Icmpv4Type::EchoReply(IcmpEchoHeader {
                        id: header.id,
                        seq: header.seq,
                    }));
                // Generate the checksum
                outgoing_icmp_header.update_checksum(&incoming_icmp_packet.payload());

                // Create a buffer for the outgoing packet
                let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

                // Write the ICMP header to the buffer
                outgoing_icmp_header.write(&mut buffer).unwrap();
                // Write the echo payload to the buffer
                buffer.write(&incoming_icmp_packet.payload()).unwrap();

                let outbound_packet: &[u8] = buffer.get_ref().as_slice();

                // This API makes no sense
                let destination_address = SockAddr::from(SocketAddrV4::new(
                    Ipv4Addr::from(incoming_ip_header.source()),
                    0,
                ));

                // Send the packet
                socket
                    .send_to(outbound_packet, &destination_address)
                    .unwrap();
                socket.flush().unwrap();
            }
            t => println!("{:?}", t),
        }

        println!("");
    }
}
