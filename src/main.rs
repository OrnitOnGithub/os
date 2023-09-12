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
    println!("Hi mom {}", 10.0/0.0);
    rust_os::init();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    let address: u32 = 0x80000000; // An address outside the allocated page
    unsafe {
        // Try to read from an invalid address
        let _value = *(address as *const u32);
    }

    println!("It did not crash!");
    loop {}
}