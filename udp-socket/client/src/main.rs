use std::io;
use std::net::UdpSocket;
use std::str::from_utf8;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8001")?;
    socket.connect("127.0.0.1:8000")?;

    let message: &[u8; 14] = b"Hello, server!";
    socket.send(message)?;

    let mut buffer: [u8; 1024] = [0u8; 1024];
    let (len, src) = socket.recv_from(&mut buffer)?;

    println!(
        "Received [{}]> {:?}",
        src,
        from_utf8(&buffer[..len]).unwrap()
    );

    Ok(())
}
