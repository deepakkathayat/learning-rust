use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln! {"failed: {}", e}
            }
            Ok(stream) => {}
        }
    }
}
