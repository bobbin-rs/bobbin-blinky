#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = PF0

// Note - See 10.3.1.2 "Data Register Operation" in Data Sheet
pub const SYSCTL_RCGCGPIO: *mut u32 = 0x400f_e608 as *mut u32;
pub const GPIOF_DATA: *mut u32 = 0x4005_d3fc as *mut u32;
pub const GPIOF_DIR: *mut u32 = 0x4005_d400 as *mut u32;
pub const GPIOF_DEN: *mut u32 = 0x4005_d51c as *mut u32;


#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable GPIOF
        ptr::write_volatile(SYSCTL_RCGCGPIO, ptr::read_volatile(SYSCTL_RCGCGPIO) | 1 << 5);
        for _ in 0..100 { asm!("nop") }
        // Set PF0 Mode = Output        
        ptr::write_volatile(GPIOF_DIR, 1 << 0);
        // Set PF0 Digital Enable
        ptr::write_volatile(GPIOF_DEN, 1 << 0);
        loop {
            // Set PF0
            ptr::write_volatile(GPIOF_DATA, 1 << 0);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset PF0
            ptr::write_volatile(GPIOF_DATA, 0 << 0);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
    }
}
