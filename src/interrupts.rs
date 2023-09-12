#![no_std]
extern crate x86_64;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::println;

use lazy_static::lazy_static;

use x86_64::instructions::hlt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, page_fault_code: PageFaultErrorCode)
{
    println!("EXCEPTION: PAGE FAULT:{:?}\n{:#?}", page_fault_code, stack_frame);
    hlt();
}