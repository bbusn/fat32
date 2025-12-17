#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod mem;
mod syscalls;

#[cfg(not(test))]
use core::panic::PanicInfo;

use syscalls::{close, exit, open, print, print_hex, read};

/* ---------- Main function ---------- */
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let path = b"test.img\0";

    let fd = open(path.as_ptr());

    if fd < 0 {
        print(b"Error when opening image file\n\0");
        exit(1);
    };

    /* ---------- Boot sector ---------- */
    let mut boot_sector = [0u8; 512];

    let r = read(fd as usize, boot_sector.as_mut_ptr(), 512);

    close(fd as usize);

    if r != 512 {
        print(b"Failed to read boot sector\n\0");
        exit(1);
    }

    if boot_sector[510] != 0x55 || boot_sector[511] != 0xAA {
        print(b"Boot sector signature is invalid\n\0");
        print(b"byte 510: \n\0");
        print_hex(boot_sector[510]);
        print(b"\n\0");
        print(b"byte 511: \n\0");
        print_hex(boot_sector[511]);
        print(b"\n\0");

        exit(1);
    }

    exit(0);
}

/* ---------- Panic handler ---------- */
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    /* If it panics, we exit */
    print(b"It panicked...\n\0");
    exit(1);
    loop {}
}

/* This is required for no_std */
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

/* ---------- Testing entry point ---------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(2 + 2, 4);
    }
}
