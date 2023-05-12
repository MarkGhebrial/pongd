use socket2::{Domain, Protocol, Socket, Type};
use daemonize::Daemonize;
use std::io;
use std::fs::File;

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
    let stdout = File::create("/tmp/pong.out").unwrap();
    let stderr = File::create("/tmp/pong.err").unwrap();

    let daemonize = Daemonize::new()
        .user("nobody")
        .group("daemon")
        .group(2)
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| open_socket().expect("Could not open socket"));

    match daemonize.start() {
        Ok(socket) => {
            println!("Listening for pings...");

            let mut echo_responder = Icmpv4EchoResponder::new(socket);

            loop {
                if let Err(e) = echo_responder.run() {
                    println!("Encountered an error: {}", e);
                }
            }
        },
        Err(e) => eprintln!("Error, {}", e),
    }
}
