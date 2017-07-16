#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PB3;

pub const RCC_IOPENR: *mut u32 = 0x4002102c as *mut u32;
pub const GPIOB_MODER: *mut u32 = 0x5000_0400 as *mut u32;
pub const GPIOB_BSRR: *mut u32 = 0x5000_0418 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable PORTB
        ptr::write_volatile(RCC_IOPENR, 1 << 1);
        // Set PB3 Mode = Output
        ptr::write_volatile(GPIOB_MODER, 0b01 << (2 * 3));
        loop {
            // Set PB3
            ptr::write_volatile(GPIOB_BSRR, 1 << 3);
            // Delay approx 1/2 second
            for _ in 0..200_000 { asm!("nop") }
            // Reset Set PB3
            ptr::write_volatile(GPIOB_BSRR, 1 << (3 + 16));
            // Delay approx 1/2 second
            for _ in 0..200_000 { asm!("nop") }
        }
    }
}
