mod dns_header;

use pretty_hex::*;
use std::net::UdpSocket;

fn udp_recv_from(socket: &UdpSocket) -> Vec<u8> {
    let mut recv_buffer = [0; 1024];
    let (recv_size, remote_addr) = socket
        .recv_from(&mut recv_buffer)
        .expect("UDP receive error");

    dbg!(recv_size);
    dbg!(remote_addr);

    return recv_buffer[..recv_size].to_owned();
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:65353").expect("Socket bind error");

    loop {
        let data = udp_recv_from(&socket);
        println!("{:?}", data.hex_dump());
    }
}
