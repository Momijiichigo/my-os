#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
use interrupts::init_idt;


// mod vga_buffer;
mod graphics;
mod logger;

pub mod interrupts;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn init() {
    init_idt();
}

#[cfg(not(test))]
entry_point!(kernel_main);
#[cfg(test)]
entry_point!(test_kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();

    let info = framebuffer.info().clone();
    logger::init_logger(framebuffer.buffer_mut(), info);
    log::info!("Hello World!");
    init();

    x86_64::instructions::interrupts::int3();
    log::info!("it did not crash");
    // for byte in framebuffer.buffer_mut() {
    //     *byte = 0x88;
    // }


    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[cfg(test)]
fn test_kernel_main() {
    init();
    test_main();
}
