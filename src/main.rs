use std::io::Read;
use std::net::{Ipv4Addr, TcpListener};
use human_bytes::human_bytes;
use reaprs::packet::Packet;

fn main() {
	let listener = TcpListener::bind((Ipv4Addr::new(127,0,0,1), 8080)).unwrap();

	let (stream, _) = listener.accept().unwrap();
	let mut sock = tungstenite::accept(stream).unwrap();
	println!("{}", "Client connected!");
	loop {
		let message = sock.read().unwrap();

		let parsed: Packet = musli_wire::decode(message.into_data().as_ref()).unwrap();

		match parsed {
			Packet::Hello => {
				println!("{}", "Got hello from client");
			}
			Packet::Goodbye => {
				println!("{}", "Received goodbye");
				break;
			}
			Packet::Allocated { size } => {
				println!("Allocated {} bytes", human_bytes(size as f64));
			}
			Packet::Deallocated { size } => {
				println!("Deallocated {} bytes", human_bytes(size as f64));
			}
		}
	}
}
