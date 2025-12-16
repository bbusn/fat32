#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

/* __________ Main function __________ */
#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
	loop {}
}

/* __________ Panic Handler __________ */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
