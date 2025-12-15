#![no_std]

use core::panic::PanicInfo;

fn main() {
    // pass
}

/* __________ Panic Handler __________ */
// Needed when doing no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// pass
	loop {}
}
