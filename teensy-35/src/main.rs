#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED = PTC5

pub const SIM_SCGC5: *mut u32 = 0x4004_8038 as *mut u32;
pub const PORTC_PCR5: *mut u32 = 0x4004_b014 as *mut u32;

pub const GPIOC_PTOR: *mut u32 = 0x400f_f08c as *mut u32;
pub const GPIOC_PDDR: *mut u32 = 0x400f_f094 as *mut u32;


#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable PORTE
        ptr::write_volatile(SIM_SCGC5, ptr::read_volatile(SIM_SCGC5) | 1 << 11);
        // Set PTC5 to GPIO
        ptr::write_volatile(PORTC_PCR5, 0x100);
        // Set PTC5 to Output
        ptr::write_volatile(GPIOC_PDDR, 1 << 5);

        loop {
            // Toggle PTB22
            ptr::write_volatile(GPIOC_PTOR, 1 << 5);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 {
                asm!("nop")
            }
        }

    }
}
