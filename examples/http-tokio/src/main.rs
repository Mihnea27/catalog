use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;

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

#[tokio::main]
async fn main() {
    let addr = format!("172.44.0.2:{}", LISTEN_PORT);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind to address");

    println!("Listening on port {}...", LISTEN_PORT);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream).await {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accept incoming connection: {}", e),
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut recvbuf = vec![0; BUFLEN];

    // Receive some bytes (ignore errors)
    let _ = stream.read(&mut recvbuf).await;

    // Send reply
    stream.write_all(REPLY.as_bytes()).await?;

    // Close connection
    stream.shutdown().await?;

    Ok(())
}

