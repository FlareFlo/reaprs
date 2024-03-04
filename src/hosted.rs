use std::alloc::System;
use std::collections::VecDeque;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::time::Duration;
use crate::packet::Packet;

pub type PacketChannel = Arc<(Mutex<VecDeque<crate::packet::Packet, System>>, Condvar)>;

pub (crate) fn spawn_sender() -> PacketChannel{
	let mut sock = TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 8080), Duration::from_secs(1)).unwrap();
	musli_wire::to_writer(&mut sock, &Packet::Hello).unwrap();
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