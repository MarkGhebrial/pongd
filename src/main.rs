use socket2::{Domain, Protocol, Socket, Type, SockAddr};
use std::net::{SocketAddrV4, Ipv4Addr};
use std::io::Read;
use std::io::Write;
use std::io::Cursor;

use std::fs::File;

use etherparse::{Icmpv4Slice, Icmpv4Type};
use etherparse::{Ipv4HeaderSlice, PacketBuilder};

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

                //dbg!(ipheader.destination(), ipheader.source());
                dbg!(header.id, header.seq);

                let builder =
                    PacketBuilder::ethernet2([0; 6], [0; 6])
                        .ipv4(ipheader.destination(), ipheader.source(), ipheader.ttl())
                        .icmpv4_echo_reply(header.id, header.seq);

                //dbg!(icmp.payload().iter().map(|c| *c as char).collect::<String>());

                dbg!(&icmp.payload()[8..]);

                //let mut buffer = File::create("test.txt").unwrap();
                let buffer = vec![];
                let mut c = Cursor::new(buffer);
                builder.write(&mut c, &icmp.payload()[16..]).unwrap();

                println!("Outbound packet: {:?}", c);

                let outbound_packet: &[u8] = c.get_ref().as_slice();

                let dest_addr = SockAddr::from(SocketAddrV4::new(Ipv4Addr::from(ipheader.source()), 0));

                socket.send_to(outbound_packet, &dest_addr).unwrap();

                //let outbound_packet: &[u8] = builder.into();

                //socket.send_to(buf)
            }
            t => println!("{:?}", t),
        }

        println!("");
    }
}
