use std::alloc::Layout;
use musli::{Encode, Decode};


#[derive(Debug, PartialEq, Encode, Decode)]
pub enum Packet {
	/// Sent when first connecting
	Hello,

	Allocated {
		size: usize,
	},

	Deallocated {
		size: usize,
	},

	/// Sent when client goes offline, causes server to terminate aswell
	Goodbye,
}