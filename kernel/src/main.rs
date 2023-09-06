#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};


// mod vga_buffer;
mod graphics;
mod logger;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();

    let info = framebuffer.info().clone();
    logger::init_logger(framebuffer.buffer_mut(), info);
    log::info!("Hello World!");
    
    // for byte in framebuffer.buffer_mut() {
    //     *byte = 0x88;
    // }
    loop {}
}

