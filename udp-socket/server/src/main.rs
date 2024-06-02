use std::net::UdpSocket;
use std::str::from_utf8;
use std::{io, process::exit};

use logger::{LogType, Logger};

fn main() {
    let mut logger = Logger::init("server.log");

    let socket: UdpSocket = match initialize() {
        Ok(socket) => {
            logger.write("Socket Created Successfully", LogType::SUCCESS);
            socket
        }
        Err(err) => {
            logger.write(&format!("Unable to create socket: {err}"), LogType::FAILURE);
            exit(1);
        }
    };

    logger.write_info("Waiting for message to be received.");
    let mut buf: [u8; 1024] = [0u8; 1024];
    let (len, src) = socket.recv_from(&mut buf).expect("Unable to read message");

    println!(
        "received [{}]> {:?}",
        &src.to_string(),
        from_utf8(&buf[..len]).unwrap()
    );

    logger.write_info("Sending message to client.");

    let len = socket
        .send_to(&buf[..len], &src)
        .expect("Unable to send data");

    logger.write(
        format!("{len} bytes sent successfully").as_str(),
        LogType::SUCCESS,
    );
}

fn initialize() -> io::Result<UdpSocket> {
    UdpSocket::bind("127.0.0.1:8000")
}
