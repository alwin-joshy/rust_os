#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]

mod framebuffer;

// use bootloader_api::{entry_point, BootInfo};
use core::{panic::PanicInfo};
use conquer_once::spin::OnceCell;
use bootloader_x86_64_common::logger::LockedLogger;
use bootloader_api::info::FrameBufferInfo;

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Kernel Mode!");
}

#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);

// ↓ this replaces the `_start` function ↓
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {

        // free the doubly wrapped framebuffer from the boot info struct
        let frame_buffer_optional = &mut boot_info.framebuffer;

        // free the wrapped framebuffer from the FFI-safe abstraction provided by bootloader_api
        let frame_buffer_option = frame_buffer_optional.as_mut();

        // unwrap the framebuffer
        let frame_buffer_struct = frame_buffer_option.unwrap();

        // extract the framebuffer info and, to satisfy the borrow checker, clone it
        let frame_buffer_info = frame_buffer_struct.info().clone();

        // get the framebuffer's mutable raw byte slice
        let raw_frame_buffer = frame_buffer_struct.buffer_mut();

        // finally, initialize the logger using the last two variables
        init_logger(raw_frame_buffer, frame_buffer_info);
    }
    loop{}
}