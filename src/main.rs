use std::io::Read;
use std::net::{Ipv4Addr, TcpListener};
use human_bytes::human_bytes;
use reaprs::packet::Packet;

fn main() {
	let listener = TcpListener::bind((Ipv4Addr::new(127,0,0,1), 8080)).unwrap();

	let (mut stream, _) = listener.accept().unwrap();
	println!("{}", "Client connected!");
	loop {
		let mut message = vec![];
		let count = stream.read_to_end(&mut message).unwrap();
		if count == 0 { continue };

		let parsed: Packet = musli_wire::decode(message.as_slice()).unwrap();

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
