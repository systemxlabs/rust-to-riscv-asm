#![no_std]
#![no_main]

extern crate alloc;

mod console;
mod lang_items;
mod heap;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    heap::init_heap();

    let sum = add_number(1, 2);
    println!("1 + 2 = {}", sum);

    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    unreachable!()
}

#[no_mangle]
#[inline(never)]
fn add_number(a: i32, b: i32) -> i32 {
    a + b
}

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}