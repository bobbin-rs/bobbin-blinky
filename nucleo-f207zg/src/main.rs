#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PB0;

pub const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
pub const GPIOB_MODER: *mut u32 = 0x4002_0400 as *mut u32;
pub const GPIOB_BSRR: *mut u32 = 0x4002_0418 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable PORTB
        ptr::write_volatile(RCC_AHB1ENR, ptr::read_volatile(RCC_AHB1ENR) | 1 << 1);
        // Set PB0 Mode = Output
        ptr::write_volatile(GPIOB_MODER, ptr::read_volatile(GPIOB_MODER) | 1 << 0);
        loop {
            // Set PB0
            ptr::write_volatile(GPIOB_BSRR, 1 << 16);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset Set PB0
            ptr::write_volatile(GPIOB_BSRR, 1 << 0);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
