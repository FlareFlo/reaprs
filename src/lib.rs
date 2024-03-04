use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use crate::hosted::spawn_sender;

use human_bytes::human_bytes;
use crate::packet::Packet;

pub struct Reaprs;

pub mod packet;
pub mod hosted;

static TRACKING: AtomicBool = AtomicBool::new(false);
static SENDER: Mutex<Option<Sender<Packet>>> = Mutex::new(None);

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
	println!("{}", "Sending");
	TRACKING.store(false, Relaxed);
	let mut lock = SENDER.lock().unwrap();
	lock.as_mut().map(|s|s.send(packet).unwrap());
	drop(lock);
	TRACKING.store(true, Relaxed);
}

pub fn init() {
	let sender = spawn_sender();
	*SENDER.lock().unwrap() = Some(sender);
	TRACKING.store(true, Relaxed);
}