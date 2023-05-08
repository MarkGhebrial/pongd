use etherparse::{IcmpEchoHeader, Icmpv4Header, Icmpv4Slice, Icmpv4Type, Ipv4HeaderSlice, WriteError};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::io::{Cursor, Read, Write, self};
use std::net::{Ipv4Addr, SocketAddrV4};

mod err;
use err::MyError;

fn open_socket() -> io::Result<Socket> {
    // Create the ICMP socket
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    // Make it blocking
    socket.set_nonblocking(false)?;

    Ok(socket)
}

fn generate_echo_reply(incoming_ip_header: &Ipv4HeaderSlice, incoming_icmp_packet: &Icmpv4Slice) -> Result<Box<[u8]>, MyError> {
    // Respond to echo requests
    match incoming_icmp_packet.icmp_type() {
        Icmpv4Type::EchoRequest(echo_request_header) => {
            println!(
                "Recieved an echo request from {} with {} bytes of data",
                Ipv4Addr::from(incoming_ip_header.source()),
                incoming_icmp_packet.slice().len()
            );

            // Create the outgoing ICMP header
            let mut outgoing_icmp_header =
                Icmpv4Header::new(Icmpv4Type::EchoReply(IcmpEchoHeader {
                    id: echo_request_header.id,
                    seq: echo_request_header.seq,
                }));
            // Generate the checksum
            outgoing_icmp_header.update_checksum(&incoming_icmp_packet.payload());

            // Create a buffer for the outgoing packet
            let mut buffer: Cursor<Box<[u8]>> = Cursor::new(Box::new([0; 4098]));

            // Write the ICMP header to the buffer
            outgoing_icmp_header.write(&mut buffer)?;
            // Write the echo payload to the buffer
            buffer.write(&incoming_icmp_packet.payload())?;

            Ok(buffer.into_inner())
        }
        _ => Err(MyError::NotAnEchoRequest)
    }
}

fn main() {
    let mut socket = open_socket().expect("Could not open socket");

    println!("Listening for pings...");

    let mut buf = [0u8; 4096];

    loop {
        // Get a packet
        let packet_len = socket.read(&mut buf).unwrap();
        let packet = &buf[..packet_len];

        // Parse the IP header
        let incoming_ip_header = Ipv4HeaderSlice::from_slice(packet).unwrap();

        // Parse the icmp header
        let incoming_icmp_packet: Icmpv4Slice = Icmpv4Slice::from_slice(
            &packet[(incoming_ip_header.total_len() - incoming_ip_header.payload_len()) as usize..],
        )
        .unwrap();

        // Respond to echo requests
        match generate_echo_reply(&incoming_ip_header, &incoming_icmp_packet) {
            Ok(packet) => {

                // This API makes no sense
                let destination_address = SockAddr::from(SocketAddrV4::new(
                    Ipv4Addr::from(incoming_ip_header.source()),
                    0,
                ));

                // Send the packet
                socket
                    .send_to(&packet, &destination_address)
                    .unwrap();
                socket.flush().unwrap();
            }
            Err(MyError::NotAnEchoRequest) => {},
            Err(e) => println!("Encountered an error: {}", e),
        }
    }
}
