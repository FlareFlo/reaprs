use std::alloc::System;
use std::collections::VecDeque;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::time::Duration;
use arrayvec::ArrayVec;
use musli::context::Buffer;
use crate::packet::Packet;

pub type PacketChannel = Arc<(Mutex<VecDeque<crate::packet::Packet, System>>, Condvar)>;

pub (crate) fn spawn_sender() -> PacketChannel{
	let mut sock = TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 8080), Duration::from_secs(1)).unwrap();
	send_packet(&mut sock, &Packet::Hello);
	let packets: PacketChannel= Arc::new((Mutex::new(VecDeque::with_capacity_in(4096, System)), Condvar::new()));

	let t = packets.clone();
	spawn(move || {
		let packets = t;
		loop {
			let mut lock = packets.1.wait(packets.0.lock().unwrap()).unwrap();

			send_packet(&mut sock,  &lock.pop_front().unwrap());
		}
	});

	return packets;
}

fn send_packet(stream: &mut TcpStream, packet: &Packet) {
	let mut buf: ArrayVec<u8, 256>  = ArrayVec::new();
	buf.write_all(&0_u64.to_ne_bytes()).unwrap();
	musli_wire::encode(buf.as_mut_slice(), &packet).unwrap();
	let len = buf.len();
	buf.write_at(0, &(len as u64).to_ne_bytes());
	stream.write_all(buf.as_slice()).unwrap();
}