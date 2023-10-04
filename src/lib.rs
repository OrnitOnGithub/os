#![no_std]
#![feature(abi_x86_interrupt)]
pub mod interrupts;
pub mod vga_buffer;
pub mod gdt;
pub mod keyboard;

// Initiate all the initiatables. Called once at system startup.
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
// Replaces the loop {} After the OS initialisation.
// Much more resource efficiet way for the Kernel to do nothing
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}