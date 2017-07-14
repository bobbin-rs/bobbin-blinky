#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// FRDM-K64F RED LED = PTB22

pub const SIM_SCGC5: *mut u32 = 0x4004_8038 as *mut u32;
pub const PORTB_PCR22: *mut u32 = 0x4004_a058 as *mut u32;

pub const GPIOB_PTOR: *mut u32 = 0x400f_f04c as *mut u32;
pub const GPIOB_PDDR: *mut u32 = 0x400f_f054 as *mut u32;


#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable PORTB
        ptr::write_volatile(SIM_SCGC5, 0x0004_0582);
        // Set PTB22 to GPIO
        ptr::write_volatile(PORTB_PCR22, 0x100);
        // Set PTB22 to Output
        ptr::write_volatile(GPIOB_PDDR, 1 << 22);

        loop {
            // Toggle PTB22
            ptr::write_volatile(GPIOB_PTOR, 1 << 22);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 {
                asm!("nop")
            }
        }

    }
}
