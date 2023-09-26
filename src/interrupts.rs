use crate::{println, print};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
extern crate x86_64;
use x86_64::instructions::hlt;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// This uses the IDT from the x86_64 crate.
//
// Lazy static can initialize static variables lazily, ensuring that
// they are only computed and initialized when they are first accessed. 
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt
    };
}

// Initialise the IDT. This function is used in lib.rs's init method.
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
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe {
        PICS.lock() //Notify both PICs
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub const PIC_1_OFFSET: u8 = 32; // Offsets PIC interrups 1-8 to interrupts 32-39
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // Same here, 1-8 to interrupts 40-47

// Initialise mutex with an instance of both PICs
//
// Mutexes are used to synchronize access to shared
// data in multi-threaded or multi-core environments,
// ensuring that only one thread can access the data at a time.
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(
        unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }
    );

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET + 0,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
