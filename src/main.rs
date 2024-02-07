use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream
        .read(&mut buffer)
        .expect("Failed to read from TCP Stream");

    // converts the data in UTF-8 encoded string.
    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Client> {}", request);

    let response = "Hello".as_bytes();

    stream
        .write(response)
        .expect("Unable to send response to client.");
}

// entry point
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind the address.");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => panic!("ERROR: {e}"),
        }
    }
}
