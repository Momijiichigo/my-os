use bootloader_api::info::{FrameBuffer, FrameBufferInfo};


use conquer_once::spin::OnceCell;
use bootloader_x86_64_common::logger::LockedLogger;

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || {
        LockedLogger::new(buffer, info, true, false)
    });

    log::set_logger(logger).expect("Logger already set");

    log::set_max_level(log::LevelFilter::Trace);

    log::info!("Hello, Kernel Mode!");
}
