use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8001")?;
    socket.connect("127.0.0.1:8000")?;

    let message: &[u8; 14] = b"Hello, server!";
    socket.send(message)?;

    let mut buffer: [u8; 1024] = [0u8; 1024];
    let (_amt, src) = socket.recv_from(&mut buffer)?;

    println!("Received [{}]> {:?}", src, &buffer.to_ascii_lowercase());

    Ok(())
}
