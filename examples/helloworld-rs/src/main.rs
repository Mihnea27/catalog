use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddrV4};
use std::str;

const LISTEN_PORT: u16 = 8080;
const REPLY: &str = "HTTP/1.1 200 OK\r\n\
                     Content-type: text/html\r\n\
                     Connection: close\r\n\
                     \r\n\
                     <!DOCTYPE HTML PUBLIC \"-//IETF//DTD HTML 2.0//EN\">\
                     <html>\
                     <head><title>It works!</title></head>\
                     <body><h1>It works!</h1><p>This is only a test.</p></body>\
                     </html>\n";
const BUFLEN: usize = 2048;

fn main() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(172, 44, 0, 2), LISTEN_PORT);
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");

    println!("Listening on port {}...", LISTEN_PORT);

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            },
            Err(e) => {
                eprintln!("Failed to accept incoming connection: {}", e);
            },
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut recvbuf = [0; BUFLEN];

    // Receive some bytes (ignore errors)
    let _ = stream.read(&mut recvbuf);

    // Send reply
    if let Err(e) = stream.write_all(REPLY.as_bytes()) {
        eprintln!("Failed to send a reply: {}", e);
    }

    // Close connection
    let _ = stream.shutdown(std::net::Shutdown::Both);
}
