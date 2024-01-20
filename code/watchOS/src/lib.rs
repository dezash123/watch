#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn PANIC(_panic_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn status_led(rgb: [u8; 3]) -> i32 {

}
