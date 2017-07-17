#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PD13;

pub const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
pub const GPIOD_MODER: *mut u32 = 0x4002_0c00 as *mut u32;
pub const GPIOD_BSRR: *mut u32 = 0x4002_0c18 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable GPIOD
        ptr::write_volatile(RCC_AHB1ENR, ptr::read_volatile(RCC_AHB1ENR) | 1 << 3);
        // Set PD13 Mode = Output
        ptr::write_volatile(GPIOD_MODER, ptr::read_volatile(GPIOD_MODER) | 1 << (13 * 2));
        loop {
            // Set PD13
            ptr::write_volatile(GPIOD_BSRR, 1 << (13 + 16));
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset Set PD13
            ptr::write_volatile(GPIOD_BSRR, 1 << 13);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
