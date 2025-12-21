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

use crate::cli::{CLI_NAME, reset_cli, print, print_bytes_hex, print_line, print_no_ln};
use crate::sys::{close, exit, open, print_bytes, read, read_at};
use boot_sector::{BootSector, parse_boot_sector, verify_boot_sector_signature};
use fat::{change_directory, list_root};

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
    reset_cli();

    let path = b"disk.img\0";
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

    list_root(fd as usize, &bs, fat_start as usize, data_start as usize);

    let mut current_cluster = bs.root_cluster;

    #[cfg(not(test))]
    loop {
	print_line();
        print_no_ln(CLI_NAME);
        print("Type 'exit' to quit or press Ctrl+C:");

        let mut buf = [0u8; 256];
        let bytes_read = unsafe { read(0, buf.as_mut_ptr(), buf.len()) };

        if bytes_read >= 4 {
            let input = &buf[..4];
            if input[0] == b'e' && input[1] == b'x' && input[2] == b'i' && input[3] == b't' {
                break;
            }
        }

        if bytes_read <= 0 {
            continue;
        }

        let mut len = bytes_read as usize;
        while len > 0
            && (buf[len - 1] == b'\n'
                || buf[len - 1] == b'\r'
                || buf[len - 1] == 0
                || buf[len - 1] == b' ')
        {
            len -= 1;
        }
        if len == 0 {
            continue;
        }

        /* Handle `cd <dir>` command */
        if len >= 3 && buf[0] == b'c' && buf[1] == b'd' && buf[2] == b' ' {
            let arg = &buf[3..len];
            match change_directory(
                fd as usize,
                &bs,
                fat_start as usize,
                data_start as usize,
                current_cluster,
                arg,
            ) {
                Some(cluster) => {
                    current_cluster = cluster;
                }
                None => {
                    print("Folder not found");
                }
            }
            continue;
        }

        /* Unknown command — show simple help */
        print("Unknown command. Use `cd <dir>` or type `exit`.");
    }
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
