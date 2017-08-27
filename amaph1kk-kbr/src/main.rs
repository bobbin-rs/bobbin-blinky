#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

pub mod lang_items;
pub mod exceptions;

use core::ptr;

// LED0 = GPIO17 / Pin F5 / Pad 43;

pub const PADREGE: *mut u32 = 0x4001_0010 as *mut u32;
pub const CFGC: *mut u32 = 0x4001_0048 as *mut u32;
pub const PADKEY: *mut u32 = 0x40010060 as *mut u32;

pub const WTSA: *mut u32 = 0x40010090 as *mut u32;
pub const WTSB: *mut u32 = 0x40010094 as *mut u32;

pub const WTCA: *mut u32 = 0x40010098 as *mut u32;
pub const WTCB: *mut u32 = 0x4001009C as *mut u32;
pub const WTEB: *mut u32 = 0x400100A4 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Set PADKEY to 0x73
        ptr::write_volatile(PADKEY, 0x73);

        // Set PADn_FNCSEL to GPIO
        let v = ptr::read_volatile(PADREGE);
        ptr::write_volatile(PADREGE, v | 0x3 << 11);
        
        // Set GPIO_GPIOCFGy to Output Push-Pull
        let v = ptr::read_volatile(CFGC);
        ptr::write_volatile(CFGC, v | 0x1 << 5);

        pub const DELAY: usize = 8_000_000;

        loop {
            // Set GPIO17
            ptr::write_volatile(WTSA, 1 << 17);
            // Delay approx 1/2 second
            for _ in 0..DELAY { asm!("nop") }
            // Clr GPIO17
            ptr::write_volatile(WTCA, 1 << 17);
            // Delay approx 1/2 second
            for _ in 0..DELAY { asm!("nop") }
        }
    }
}
