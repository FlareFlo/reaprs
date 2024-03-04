use reaprs::{Reaprs};

#[global_allocator]
static A: Reaprs = Reaprs;

fn main() {
    reaprs::init();
    let mut allocs = vec![];

    for i in 0..fastrand::usize(0..10_000) {
        let alloc: Vec<u8> = Vec::with_capacity(i);
        allocs.push(alloc);
    }
}
