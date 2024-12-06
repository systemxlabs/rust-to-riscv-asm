use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;

use crate::println;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::new();

pub fn init_heap() {
    extern "C" {
        fn image_end();
    }
    unsafe {
        // 1G
        let heap_size = 1024 * 1024 * 1024;
        HEAP_ALLOCATOR.lock().init(image_end as usize, heap_size);
        // println!("inited heap: [{:#x}, {:#x})", image_end as usize, image_end as usize + heap_size);
    }

    // heap_test();
}

#[allow(dead_code)]
fn heap_test() {
    let mut v = Vec::new();
    for i in 0..100 {
        v.push(i);
    }
    assert_eq!(v.len(), 100);
    assert_eq!(v[0], 0);
    assert_eq!(v[99], 99);
    // println!("v: {:?}", v);
}