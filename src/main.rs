#![no_std]
#![no_main]

mod mem;
mod syscalls;

use core::panic::PanicInfo;
use syscalls::{exit, open, read, close};

/* ---------- Main function ---------- */
#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let path = b"test.img\0";
    
    let fd = open(path.as_ptr());

    if fd < 0 {
        exit(1);
    };


    let mut boot_sector = [0u8; 512];

    let r = read(fd as usize, boot_sector.as_mut_ptr(), 512);

    close(fd as usize);

    if r < 0 {
        exit(1);
    }

    exit(0);
}

/* We need to implement this panic handler in no_std */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    /* If it panic we only exit */
    exit(1);
    loop {}
}

/* This is not called but required when no_std so the compiler don't complain */
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}
