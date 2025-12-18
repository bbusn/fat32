#![no_std]
#![no_main]

#[cfg(test)]
extern crate std;

mod cli;
mod sys;

#[cfg(not(test))]
use core::panic::PanicInfo;

use crate::cli::{init_cli, print, print_bytes_hex};
use crate::sys::{close, exit, open, read};

// When not testing, we need this func to call main for aarch64
#[cfg(not(test))]
#[unsafe(no_mangle)]
fn __libc_start_main() {
    main();
}

// When not testing, we need this func for compiler for aarch64
#[cfg(not(test))]
#[unsafe(no_mangle)]
fn abort() {
    exit(1);
}

/* ---------- Main function ---------- */
#[unsafe(no_mangle)]
fn main() {
    init_cli();

    let path = b"disk.img\0";

    let fd = open(path.as_ptr());

    if fd < 0 {
        print("Error when opening image disk.img file, are you at the root directory?");
        exit(1);
    };

    /* ---------- Boot sector ---------- */
    let mut boot_sector = [0u8; 512];

    let r = read(fd as usize, boot_sector.as_mut_ptr(), 512);

    close(fd as usize);

    if r != 512 {
        print("Failed to read boot sector");
        exit(1);
    }

    if boot_sector[510] != 0x55 || boot_sector[511] != 0xAA {
        print("Boot sector signature is invalid");
        print_bytes_hex(&boot_sector[510..512]);

        exit(1);
    }

    // loop {}
    exit(0);
}

/* We need to implement this panic handler in no_std */
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    /* If it panic we exit */
    print("It panicked...");
    exit(1);
    loop {}
}

/* This is not called but required when no_std so the compiler don't complain */
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}
