use etherparse::{IcmpEchoHeader, Icmpv4Header, Icmpv4Slice, Icmpv4Type, Ipv4HeaderSlice};
use socket2::{SockAddr, Socket};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use bytes::{Bytes, BytesMut};
use bytes::buf::BufMut;

use crate::MyError;

pub struct Icmpv4EchoResponder {
    socket: Socket,
    buffer: Bytes,
}

impl Icmpv4EchoResponder {
    pub fn new(socket: Socket) -> Self {
        Self { socket, buffer: Bytes::new() }
    }

    /// Blocks until the socket returns an IP packet
    fn get_packet(&mut self) -> Result<(), MyError> {
        let mut buf = [0u8; 4098];

        // Get a packet
        let packet_len = self.socket.read(&mut buf)?;

        self.buffer = Bytes::copy_from_slice(&buf[..packet_len]);

        Ok(())
    }

    fn generate_echo_reply(
        incoming_ip_header: &Ipv4HeaderSlice,
        incoming_icmp_packet: &Icmpv4Slice,
    ) -> Result<Bytes, MyError> {
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
                //let mut buffer: Cursor<Box<[u8]>> = Cursor::new(Box::new([0; 4098]));
                let buffer = BytesMut::with_capacity(4098);
                let mut writer = buffer.writer();

                // Write the ICMP header to the buffer
                outgoing_icmp_header.write(&mut writer)?;
                // Write the echo payload to the buffer
                writer.write(&incoming_icmp_packet.payload())?;

                Ok(writer.into_inner().into())
            }
            _ => Err(MyError::NotAnEchoRequest),
        }
    }

    pub fn run(&mut self) -> Result<(), MyError> {
        self.get_packet()?;

        // Parse the IP header
        let ip_header = Ipv4HeaderSlice::from_slice(&self.buffer)?;

        // Parse the icmp header
        let icmp_packet: Icmpv4Slice = Icmpv4Slice::from_slice(
            &self.buffer[(ip_header.total_len() - ip_header.payload_len()) as usize..],
        )?;

        let outboud_packet = match Self::generate_echo_reply(&ip_header, &icmp_packet) {
            Ok(bytes) => bytes,
            Err(MyError::NotAnEchoRequest) => return Ok(()),
            Err(e) => return Err(e),
        };

        // Send echo reply to the origin of the echo request
        let destination_address =
            SockAddr::from(SocketAddrV4::new(Ipv4Addr::from(ip_header.source()), 0));

        // Send the echo reply packet
        self.socket
            .send_to(&outboud_packet, &destination_address)
            .unwrap();
        self.socket.flush().unwrap();

        Ok(())
    }
}
