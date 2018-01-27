#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(asm)]
#![feature(ptr_internals)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use] mod vga_buffer;
#[macro_use] mod portio;
#[macro_use] mod log;
mod serial;

#[no_mangle]
pub extern fn kernel_main() {
	// ATTENTION: we have a very small stack and no guard page

    vga_buffer::clear_screen();
    println!("Hello world{}", "!");

    logln!("ASDF: 0x{0:X}", 0x1234);

    println!("After serial");

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() { }
#[lang = "panic_fmt"] #[no_mangle] extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
	println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
	loop { }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    println!("\n\nUnwind resume called");
    loop {}
}
