#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED1 = PA8;

pub const PIN: usize = 8;

pub const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
pub const GPIOA_MODER: *mut u32 = 0x4002_0000 as *mut u32;
pub const GPIOA_BSRR: *mut u32 = 0x4002_0018 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable GPIOA
        ptr::write_volatile(RCC_AHB1ENR, ptr::read_volatile(RCC_AHB1ENR) | 1 << 0);
        // Set PA8 Mode = Output
        ptr::write_volatile(GPIOA_MODER, ptr::read_volatile(GPIOA_MODER) | 0b01 << (PIN * 2));
        loop {
            // Set PA8
            ptr::write_volatile(GPIOA_BSRR, 1 << (PIN + 16));
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset Set PD13
            ptr::write_volatile(GPIOA_BSRR, 1 << PIN);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
