use std::io::{Error, Read, Write};
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();

                println!(
                    "Connection established: {}",
                    String::from_utf8_lossy(&buffer[..])
                );
                let response = "HTTP/1.1 200 OK\r\n\r\n";

                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}
