use std::net::UdpSocket;
use std::{thread, time};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:9999").unwrap();
    let buf = [1u8; 15000];
    let count = 1473;
    socket.send_to(&buf[0..count], "234.2.2.2:8888").unwrap();

    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}
