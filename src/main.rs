#![no_std] 
#![no_main] 
#![feature(alloc_error_handler)] 
#![feature(abi_x86_interrupt)]

// #![feature(alloc)]
pub mod allocator;
pub mod interrupts;
pub mod vga_buffer;
pub mod gdt;
pub mod memory;
use crate::memory::*;

extern crate alloc;
extern crate rlibc;
use core::panic::PanicInfo;
use alloc::vec;
// use alloc::string::String;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

// this function halts the process. ref: https://github.com/rust-osdev/x86_64/blob/5e8e218381c5205f5777cb50da3ecac5d7e3b1ab/src/instructions/mod.rs#L16-L22
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; 
    x86_64::instructions::interrupts::enable();
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    init();
    // let n = vec!["a"];
    println!("Started the Radence Kernel. (c) 2020 https://codic12.github.io");
    // loop {
        print!("> ");
    // }

    hlt_loop(); 
}