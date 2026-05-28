mod answer;
mod header;
mod question;

use anyhow::Result;

use crate::{answer::Answer, header::Header, question::Question};
use std::net::UdpSocket;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let received_header = Header::from_bytes(buf[..12].try_into()?);
                let question_vec = Question::from_bytes(&buf, received_header.qdcount)?;
                let mut response = Vec::new();

                let header = Header::new(
                    received_header.id,
                    1,
                    received_header.opcode,
                    0,
                    0,
                    received_header.rd,
                    0,
                    0,
                    if received_header.opcode == 0 { 0 } else { 4 },
                    received_header.qdcount,
                    received_header.qdcount,
                    0,
                    0,
                );
                response.extend(header.to_bytes());

                for question in &question_vec {
                    response.extend(question.to_bytes());
                }

                for question in question_vec {
                    let answer = Answer::new(question.name, 1, 1, 60, 4, vec![8, 8, 8, 8]);
                    response.extend(answer.to_bytes());
                }

                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                return Ok(());
            }
        }
    }
}
