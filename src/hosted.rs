use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;
use std::time::Duration;
use crate::packet::Packet;

pub (crate) fn spawn_sender() -> Sender<Packet> {
	let mut sock = TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 8080), Duration::from_secs(1)).unwrap();
	sock.write_all(&musli_wire::to_vec(&Packet::Hello).unwrap()).unwrap();
	let (sender,receiver) = channel();

	spawn(move || {
		let receiver = receiver;

		loop {
			let packet = receiver.recv().unwrap();

			match packet {

				_ => {},
			}
		}
	});

	return sender;
}