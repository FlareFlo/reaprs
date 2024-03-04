#![feature(allocator_api)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{OnceLock};
use crate::hosted::spawn_sender;
use crate::hosted::PacketChannel;

use crate::packet::Packet;

pub struct Reaprs;

pub mod packet;
pub mod hosted;

static TRACKING: AtomicBool = AtomicBool::new(false);
static SENDER: OnceLock<PacketChannel> = OnceLock::new();

unsafe impl GlobalAlloc for Reaprs {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let ret = System.alloc(layout);
		if !ret.is_null() {
			if TRACKING.load(Relaxed) {
				send_packet(Packet::Allocated {size: layout.size()});
			}
		}
		ret
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		System.dealloc(ptr, layout);
		if TRACKING.load(Relaxed) {
			send_packet(Packet::Deallocated {size: layout.size()});
		}
	}
}

fn send_packet(packet: Packet) {
	let cell = SENDER.get().unwrap();
	let mut lock = cell.0.lock().unwrap();
	lock.push_back(packet);
	cell.1.notify_all();

}

pub fn init() {
	let sender = spawn_sender();
	SENDER.set(sender).unwrap();
	TRACKING.store(true, Relaxed);
}