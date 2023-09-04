// compile with: cargo build --target x86_64-rust_os.json
// compile rust std for bare metal: cargo build -Z build-std --target x86_64-rust_os.json
// qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin

#![no_std]
#![no_main]

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
    println!("Hello {}", 1.0/3.0);
    loop {} 
}