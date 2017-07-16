#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// FRDM-KL26Z LED = PTE29

pub const SIM_SCGC5: *mut u32 = 0x4004_8038 as *mut u32;
pub const PORTE_PCR29: *mut u32 = 0x4004_d074 as *mut u32;

pub const GPIOE_PTOR: *mut u32 = 0x400f_f10c as *mut u32;
pub const GPIOE_PDDR: *mut u32 = 0x400f_f114 as *mut u32;


#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable PORT#
        ptr::write_volatile(SIM_SCGC5, ptr::read_volatile(SIM_SCGC5) | 1 << 13);
        // Set PTE29 to GPIO
        ptr::write_volatile(PORTE_PCR29, 0x100);
        // Set PTE29 to Output
        ptr::write_volatile(GPIOE_PDDR, 1 << 29);

        loop {
            // Toggle PTB22
            ptr::write_volatile(GPIOE_PTOR, 1 << 29);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 {
                asm!("nop")
            }
        }

    }
}
