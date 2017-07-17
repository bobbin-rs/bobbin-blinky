#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PC13;

pub const RCC_APB2ENR: *mut u32 = 0x4002_1018 as *mut u32;
pub const RCC_APB2ENR_IOPCEN: u32 = 1 << 4;
pub const GPIOC_CRH: *mut u32 = 0x4001_1004 as *mut u32;
pub const GPIOC_BSRR: *mut u32 = 0x4001_1010 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable GPIOC
        ptr::write_volatile(RCC_APB2ENR, ptr::read_volatile(RCC_APB2ENR) | RCC_APB2ENR_IOPCEN);
        // Set PC13 Mode = Output
        ptr::write_volatile(GPIOC_CRH, 0x44544444);
        loop {
            // Set PC13
            ptr::write_volatile(GPIOC_BSRR, 1 << (13 + 16));
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset Set PC13
            ptr::write_volatile(GPIOC_BSRR, 1 << 13);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
