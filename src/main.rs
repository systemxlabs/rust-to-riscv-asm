#![no_std]
#![no_main]

extern crate alloc;

mod console;
mod lang_items;
mod heap;

static BOOT_STACK_SIZE: usize = 10 * 1024 * 1024;  // 10M
static BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0u8; BOOT_STACK_SIZE];

core::arch::global_asm!("
    .section .text.entry
    .global _start
_start:
    la sp, {boot_stack}
    li t0, {boot_stack_size}
    add sp, sp, t0
    call rust_main
",
boot_stack = sym BOOT_STACK,
boot_stack_size = const BOOT_STACK_SIZE
);

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