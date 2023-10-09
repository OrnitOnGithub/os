#![no_std]
#![no_main]
extern crate x86_64;
mod vga_buffer;

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hi mom");
    println!("Initialising operating system.");
    rust_os::init();
    println!("Done!");
    rust_os::hlt_loop(); 
}
