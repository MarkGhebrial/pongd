use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4};

use std::fs::File;

use etherparse::Ipv4HeaderSlice;
use etherparse::{IcmpEchoHeader, Icmpv4Header, Icmpv4Slice, Icmpv4Type, Icmpv6Type, Ipv4Header, IpNumber};

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
        let incoming_ip_header = Ipv4HeaderSlice::from_slice(packet).unwrap();

        // Parse the icmp header
        let icmp = Icmpv4Slice::from_slice(
            &packet[(incoming_ip_header.total_len() - incoming_ip_header.payload_len()) as usize..],
        )
        .unwrap();

        match icmp.icmp_type() {
            Icmpv4Type::EchoRequest(header) => {
                println!("THAT'S AN ECHO REQUEST!!! {:?}", header);

                let data_len = icmp.payload().len() - icmp.header_len();
                println!("INCOMING PAYLOAD LEN {}", icmp.payload().len() - icmp.header_len());
                //println!("PAYLOAD {:?}", &icmp.payload());

                let mut icmp_header = Icmpv4Header::new(Icmpv4Type::EchoReply(IcmpEchoHeader {
                    id: header.id,
                    seq: header.seq,
                }));

                icmp_header.update_checksum(&icmp.payload());

                // let dest_addr =
                //     SockAddr::from(SocketAddrV4::new(Ipv4Addr::from(incoming_ip_header.source()), 0));

                let ip_header = Ipv4Header::new(
                    (data_len + icmp_header.header_len()) as u16,
                    incoming_ip_header.ttl(),
                    IpNumber::Icmp as u8,
                    incoming_ip_header.destination(),
                    incoming_ip_header.source()
                );
                
                let buffer: Vec<u8> = vec![];
                let mut c = Cursor::new(buffer);

                // Write the ip and icmp headers to the buffer
                //ip_header.write(&mut c).unwrap();
                //c.set_position(ip_header.header_len() as u64);

                icmp_header.write(&mut c).unwrap();

                // Write the icmp payload to the buffer
                //c.write(&icmp.bytes5to8()).unwrap();
                c.write(&icmp.payload()).unwrap();

                //println!("{:?}", c);
                //println!("ICMP Payload: {:?}", &icmp.payload());

                //builder.write(&mut c, &icmp.payload()[16..]).unwrap();

                let outbound_packet: &[u8] = c.get_ref().as_slice();

                println!("SENDING {:?}", outbound_packet);

                let dest_addr =
                    SockAddr::from(SocketAddrV4::new(Ipv4Addr::from(incoming_ip_header.source()), 0));

                socket.send_to(outbound_packet, &dest_addr).unwrap();
                socket.flush().unwrap();



                //let outbound_packet: &[u8] = builder.into();

                //socket.send_to(buf)
            }
            t => println!("{:?}", t),
        }

        println!("");
    }
}
