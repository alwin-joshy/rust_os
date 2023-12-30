#![no_std]
#![no_main]

use core::*;

// use bootloader_api::{entry_point, BootInfo};
use core::{panic::PanicInfo};

#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);

// ↓ this replaces the `_start` function ↓
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x90;
        }
        // let info = framebuffer.info();
        // let buffer = framebuffer.buffer();
    }
    loop{}
}