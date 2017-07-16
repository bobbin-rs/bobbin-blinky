#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PA5;

pub const RCC_AHBENR: *mut u32 = 0x4002_1014 as *mut u32;
pub const GPIOA_MODER: *mut u32 = 0x4800_0000 as *mut u32;
pub const GPIOA_BSRR: *mut u32 = 0x4800_0018 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable PORTA
        ptr::write_volatile(RCC_AHBENR, ptr::read_volatile(RCC_AHBENR) | 1 << 17);
        // Set PA5 Mode = Output
        ptr::write_volatile(GPIOA_MODER, ptr::read_volatile(GPIOA_MODER) | (0b01 << 10));
        loop {
            // Set PA5
            ptr::write_volatile(GPIOA_BSRR, 1 << (5 + 16));
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset Set PA5
            ptr::write_volatile(GPIOA_BSRR, 1 << 5);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
