use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[ERROR] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("[ERROR] Panicked: {}", info.message());
    }
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::SystemFailure);
    unreachable!()
}
