#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
mod uart;

#[no_mangle]
pub extern "C" fn _starta() -> ! {
    println!("testing");
    loop {} 
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}