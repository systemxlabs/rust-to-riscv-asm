use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;

use crate::println;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::new();

pub fn init_heap() {
    extern "C" {
        fn free_mem_start();
    }
    unsafe {
        // 1G
        let heap_size = 1024 * 1024 * 1024;
        HEAP_ALLOCATOR.lock().init(free_mem_start as usize, heap_size);
        println!("inited heap: [{:#x}, {:#x})", free_mem_start as usize, free_mem_start as usize + heap_size);
    }

    heap_test();
}

#[allow(dead_code)]
fn heap_test() {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
    }
    assert_eq!(v.len(), 1000);
    assert_eq!(v[0], 0);
    assert_eq!(v[99], 99);
    assert_eq!(v[999], 999);
    println!("v: {:?}", v);
}