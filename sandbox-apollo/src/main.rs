#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = GPIO43 / Pin F5 / Pad 43;

pub const PADREGK: *mut u32 = 0x4001_0028 as *mut u32;
pub const CFGF: *mut u32 = 0x40010054 as *mut u32;
pub const PADKEY: *mut u32 = 0x40010060 as *mut u32;
pub const WTSB: *mut u32 = 0x40010094 as *mut u32;
pub const WTCB: *mut u32 = 0x4001009C as *mut u32;
pub const WTEB: *mut u32 = 0x400100A4 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Set PADKEY to 0x73
        ptr::write_volatile(PADKEY, 0x73);

        // Set PADn_FNCSEL to GPIO
        ptr::write_volatile(PADREGK, 0x18000000);
        
        // Set GPIO_GPIOCFGy to Output Push-Pull
        ptr::write_volatile(CFGF, 0x2000);

        loop {
            // Set GPIO43
            ptr::write_volatile(WTSB, 1 << (43 - 32));
            // Delay approx 1/2 second
            for _ in 0..200_000 { asm!("nop") }
            // Clr GPIO43
            ptr::write_volatile(WTCB, 1 << (43 - 32));
            // Delay approx 1/2 second
            for _ in 0..200_000 { asm!("nop") }
        }
    }
}
