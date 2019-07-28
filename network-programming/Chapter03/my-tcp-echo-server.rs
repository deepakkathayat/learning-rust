use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
                println!("Connection established!");
            }
        }
    }
}
