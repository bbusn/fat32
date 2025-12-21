#![no_std]
#![no_main]

#[cfg(test)]
extern crate std;

mod boot_sector;
mod cli;
mod fat;
mod helpers;
mod sys;

#[cfg(not(test))]
use core::panic::PanicInfo;

use crate::cli::{init_cli, print_line, print, print_bytes_hex, print_no_ln, CLI_NAME};
use crate::sys::{close, exit, open, print_bytes, read, read_at};
use boot_sector::{BootSector, parse_boot_sector, verify_boot_sector_signature};

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

fn wait_for_input() {
    let mut buf = [0u8; 256];
    loop {
        let bytes_read = unsafe { read(0, buf.as_mut_ptr(), buf.len()) };
        if bytes_read >= 4 {
            let input = &buf[..4];
            if input[0] == b'e' && input[1] == b'x' && input[2] == b'i' && input[3] == b't' {
                break;
            }
        }
    }
}

/* ---------- Main function ---------- */
#[unsafe(no_mangle)]
fn main() {
    init_cli();

    let path = b"disk.img\0";
    print_bytes(path);
    print("\n");

    let fd = open(path.as_ptr());

    if fd < 0 {
        print("Error when opening image disk.img file, are you at the root directory?");
        exit(1);
    };

    /* ---------- Boot sector ---------- */
    let mut boot_sector = [0u8; 512];

    let r = read(fd as usize, boot_sector.as_mut_ptr(), 512);

    if r != 512 {
        print("Failed to read boot sector");
        exit(1);
    }

    if verify_boot_sector_signature(&boot_sector) != true {
        print("Boot sector signature is invalid");
        print_bytes_hex(&boot_sector[510..512]);

        exit(1);
    }

    let bs: BootSector = parse_boot_sector(&boot_sector);

    /* ---------- Localize FATs ---------- */
    let fat_start = (bs.reserved_sectors_count as u32) * (bs.bytes_per_sector as u32);
    let data_start =
        fat_start + (bs.fats_count as u32) * bs.fat_size_sectors * bs.bytes_per_sector as u32;

    fat::list_root(fd as usize, &bs, fat_start as usize, data_start as usize);
    print_line();
    print_no_ln(CLI_NAME);
    print("Type 'exit' to quit or press Ctrl+C:");
    wait_for_input();

    close(fd as usize);

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
