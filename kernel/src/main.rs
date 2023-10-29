#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

// mod vga_buffer;
mod graphics;
mod logger;
mod gdt;

pub mod interrupts;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct InitConfig {
    boot_info: &'static mut BootInfo,
}

/// Initializes the kernel.
fn init(config: InitConfig) {
    // initializes Global Descriptor Table
    gdt::init();

    // initializes the exception handling and interrupts
    interrupts::init_idt();

    // initializes the logger
    // so that `log::info!`, etc. will be available to use
    let framebuffer = config.boot_info.framebuffer.as_mut().unwrap();
    let framebuffer_info = framebuffer.info().clone();
    logger::init_logger(framebuffer.buffer_mut(), framebuffer_info);


}

#[cfg(not(test))]
entry_point!(kernel_main);
#[cfg(test)]
entry_point!(test_kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(InitConfig { boot_info });

    log::info!("Hello World!");

    // x86_64::instructions::interrupts::int3();
    // log::info!("it did not crash");

    // for byte in framebuffer.buffer_mut() {
    //     *byte = 0x88;
    // }

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();
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
fn test_kernel_main(boot_info: &'static mut BootInfo) {
    init(InitConfig { boot_info });
    test_main();
}
