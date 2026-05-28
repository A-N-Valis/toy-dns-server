mod header;
mod question;

use anyhow::{Result, bail};

use crate::{
    header::Header,
    question::{Question, parse_name},
};
use std::{env, net::UdpSocket};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let resolver_addr = if args.len() >= 3 && args[1] == "--resolver" {
        args[2].clone()
    } else {
        bail!("missing resolver address")
    };

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let resolver_socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind addresss");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let request_packet = &buf[..size];

                let received_header = Header::from_bytes(request_packet[..12].try_into()?);
                let question_vec = Question::from_bytes(request_packet, received_header.qdcount)?;
                let mut answer_vec = Vec::new();

                for question in &question_vec {
                    let mut forward_packet = Vec::new();

                    let forward_header = Header::new(
                        received_header.id,
                        0,
                        received_header.opcode,
                        0,
                        0,
                        received_header.rd,
                        0,
                        0,
                        0,
                        1,
                        0,
                        0,
                        0,
                    );

                    forward_packet.extend(forward_header.to_bytes());
                    forward_packet.extend(question.to_bytes());

                    resolver_socket.send_to(&forward_packet, &resolver_addr)?;

                    let mut resolver_buf = [0u8; 512];

                    let (resolver_size, _) = resolver_socket.recv_from(&mut resolver_buf)?;
                    let resolver_response = &resolver_buf[..resolver_size];

                    let resolver_header = Header::from_bytes(resolver_response[..12].try_into()?);
                    let mut current_pos = 12;

                    for _ in 0..resolver_header.qdcount {
                        let (_, next_pos) = parse_name(resolver_response, current_pos)?;
                        current_pos = next_pos + 4;
                    }

                    let resolver_answer_start = current_pos;
                    let answer_bytes = resolver_response[resolver_answer_start..].to_vec();

                    answer_vec.push(answer_bytes);
                }

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
                    answer_vec.len() as u16,
                    0,
                    0,
                );
                response.extend(header.to_bytes());

                for question in &question_vec {
                    response.extend(question.to_bytes());
                }

                for answer in answer_vec {
                    response.extend(answer);
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
