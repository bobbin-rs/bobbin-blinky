#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PA17

pub const PORTA_DIR: *mut u32 = 0x41004400 as *mut u32;
pub const PORTA_OUTTGL: *mut u32 = 0x4100441c as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Set PA17 DIR = OUTPUT
        ptr::write_volatile(PORTA_DIR, 1 << 17);
        loop {
            // Toggle PA17 Output
            ptr::write_volatile(PORTA_OUTTGL, 1 << 17);
            // Delay approx 1/2 second
            for _ in 0..200_000 { asm!("nop") }
        }
    }
}
