// compile with: cargo build --target x86_64-rust_os.json
// compile rust std for bare metal: cargo build -Z build-std --target x86_64-rust_os.json
// qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin

#![no_std]
#![no_main]
//#![feature(abi_x86_interrupt)]

extern crate x86_64;
//use x86_64::structures::idt::InterruptStackFrame;

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
    let invalid_address = 0xdeadbeec as *const u32;
    unsafe {
        let _value = *invalid_address;
    }
    loop {} 
}


/* 
// Define the Page Fault handler function
#[allow(experimental)]
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    // Print the error code using println!
    println!("Page Fault Exception - Error Code: 0x{:X}", error_code);

    // Additional handling logic can be added here if needed.
    return;
}
*/