#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED = P0_19

pub const PIN: usize = 19;

pub const GPIO_PIN_CNF19: *mut u32 = 0x5000074c as *mut u32;
pub const GPIO_OUTSET: *mut u32 = 0x50000508 as *mut u32;
pub const GPIO_OUTCLR: *mut u32 = 0x5000050c as *mut u32;
pub const GPIO_DIRSET: *mut u32 = 0x50000518 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Set P0_19 DIR = OUTPUT
        ptr::write_volatile(GPIO_DIRSET, 1 << PIN);
        loop {
            // Set P0_19 Output
            ptr::write_volatile(GPIO_OUTSET, 1 << PIN);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Set P0_19 Output
            ptr::write_volatile(GPIO_OUTCLR, 1 << PIN);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
