#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PB13;

pub const RCC_AHBENR: *mut u32 = 0x4002_1014 as *mut u32;
pub const GPIOB_MODER: *mut u32 = 0x4800_0400 as *mut u32;
pub const GPIOB_BSRR: *mut u32 = 0x4800_0418 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable GPIOB
        ptr::write_volatile(RCC_AHBENR, ptr::read_volatile(RCC_AHBENR) | 1 << 18);

        // Set PB13 Mode = Output
        ptr::write_volatile(GPIOB_MODER, ptr::read_volatile(GPIOB_MODER) | (0b01 << (13 * 2)));
        loop {
            // Set PB13
            ptr::write_volatile(GPIOB_BSRR, 1 << (13 + 16));
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset Set PB13
            ptr::write_volatile(GPIOB_BSRR, 1 << 13);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
