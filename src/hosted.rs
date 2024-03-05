use std::alloc::System;
use std::collections::VecDeque;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::time::Duration;
use tungstenite::Message;
use crate::packet::Packet;

pub type PacketChannel = Arc<(Mutex<VecDeque<crate::packet::Packet, System>>, Condvar)>;

pub (crate) fn spawn_sender() -> PacketChannel{
	let (mut sock, response) = tungstenite::connect("ws://localhost:8080").unwrap();
	let mut msg: Vec<u8, System> = Vec::with_capacity_in(16, System);
	musli_wire::encode(&mut msg,&Packet::Hello).unwrap();
	sock.write(Message::Binary(msg));
	let packets: PacketChannel= Arc::new((Mutex::new(VecDeque::with_capacity_in(4096, System)), Condvar::new()));

	let t = packets.clone();
	spawn(move || {
		let packets = t;
		loop {
			let mut lock = packets.1.wait(packets.0.lock().unwrap()).unwrap();

			musli_wire::to_writer(&mut sock, &lock.pop_front().unwrap()).unwrap();
		}
	});

	return packets;
}