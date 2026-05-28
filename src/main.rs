mod header;
mod question;

use crate::{header::Header, question::Question};
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let mut response = Vec::new();

                let header = Header::new(1234, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0);
                response.extend(header.to_bytes());

                let question = Question::new("codecrafters.io".to_string(), 1, 1);
                response.extend(question.to_bytes());

                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
