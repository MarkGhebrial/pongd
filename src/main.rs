use socket2::{Domain, Protocol, Socket, Type};
use std::io;

mod err;
use err::MyError;

mod echo_responder;
use echo_responder::Icmpv4EchoResponder;

fn open_socket() -> io::Result<Socket> {
    // Create the ICMP socket
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    // Make it blocking
    socket.set_nonblocking(false)?;

    Ok(socket)
}

fn main() {
    let socket = open_socket().expect("Could not open socket");

    println!("Listening for pings...");

    let mut echo_responder = Icmpv4EchoResponder::new(socket);

    loop {
        if let Err(e) = echo_responder.run() {
            println!("Encountered an error: {}", e);
        }
    }
}
