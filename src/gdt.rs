// global Descriptor Table

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// TSS - Task State Segment
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new(); 
        // Set up the IST entry for the double fault
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5; // Define the size of the stack in bytes (4096 bytes * 5 = 20 KB)
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; // Create the stack as static mutable array

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK }); // Calculate the starting virtual address of the stack
            let stack_end = stack_start + STACK_SIZE;
            stack_end // Return the ending virtual address, which will be used as the IST entry
        };
        tss
    };
}

// GDT - Global Descriptor Table
lazy_static! {
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment()); // God knows?
        gdt.add_entry(Descriptor::tss_segment(&TSS)); // Give it our TSS
        gdt
    };
}

pub fn init() {
    GDT.load();
}