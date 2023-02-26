use std::net::UdpSocket;
use std::time::Duration;

use rustdns::{Class, Extension, Message, Record, Type};

use crate::error::WebSocketError;

pub fn get_mx_records(domain: &str) -> Result<Vec<Record>, WebSocketError> {
    let mut message = Message::default();
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let mut response = [0; 4096];


    message.add_question(domain, Type::MX, Class::Internet);
    message.add_extension(Extension::default());

    socket.set_read_timeout(Some(Duration::new(5, 0)))?;
    socket.connect("1.1.1.1:53")?;
    socket.send(&message.to_vec()?)?;

    let response_length = socket.recv(&mut response)?;
    let answer = Message::from_slice(&response[0..response_length])?;

    Ok(answer.answers)
}
