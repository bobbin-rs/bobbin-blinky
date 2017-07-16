#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// Blue LED = PTD0

pub const PCC_PORTD: *mut u32 = 0x4006_5130 as *mut u32;
pub const PORTD_PCR0: *mut u32 = 0x4004_c000 as *mut u32;

pub const GPIOD_PTOR: *mut u32 = 0x400f_f0cc as *mut u32;
pub const GPIOD_PDDR: *mut u32 = 0x400f_f0d4 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable PORTD in PCC
        ptr::write_volatile(PCC_PORTD, 1 << 30);
        // Set PTD0 to GPIO
        ptr::write_volatile(PORTD_PCR0, 0x100);
        // Set PTD0 to Output
        ptr::write_volatile(GPIOD_PDDR, 1 << 0);

        loop {
            // Toggle PTD0
            ptr::write_volatile(GPIOD_PTOR, 1 << 0);
            // Delay approx 1/2 second
            for _ in 0..4_000_000 {
                asm!("nop")
            }
        }
    }
}
