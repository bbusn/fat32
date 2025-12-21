use crate::cli::consts::{HEX, CLI_NAME};
use crate::sys::print_bytes;

/* __________ Helpers __________ */
pub fn print(val: &str) {
    print_bytes(val.as_bytes());
    print_bytes(b"\n");
}

pub fn print_no_ln(val: &str) {
    print_bytes(val.as_bytes());
}

// Convert a byte to hex
pub fn byte_to_hex(byte: u8) -> [u8; 3] {
    [HEX[(byte >> 4) as usize], HEX[(byte & 0x0F) as usize], b' ']
}

// Print a single byte as hex
pub fn print_hex(byte: u8) {
    let buf = byte_to_hex(byte);
    print_bytes(&buf);
}

// Print an entire byte slice as hex
pub fn print_bytes_hex(bytes: &[u8]) {
    for &b in bytes {
        print_hex(b);
    }

    print_bytes(b"\n");
}

pub fn clear_cli() {
    print_bytes(b"\x1B[H\x1B[2J\x1B[3J\x1B[0m");
}

pub fn print_ascii() {
    print(
        r"
          __       _   _________                _      _
         / _| __ _| |_|___ /___ \            __| |_ __(_)_   _____ _ __
        | |_ / _` | __| |_ \ __) |  _____   / _` | '__| \ \ / / _ \ '__|
        |  _| (_| | |_ ___) / __/  |_____| | (_| | |  | |\ V /  __/ |
        |_|  \__,_|\__|____/_____|          \__,_|_|  |_| \_/ \___|_|
        ",
    );
    print("\n");
}

pub fn print_line() {
    print("_______________________________________________________________________________");
}

pub fn print_ls(entry: &[u8], is_dir: bool, last: bool, indent_level: usize) {
    for _ in 0..indent_level {
        print_bytes(b"|   ");
    }

    if last {
        print_bytes(b"\xE2\x94\x94\xE2\x94\x80 ");
    } else {
        print_bytes(b"\xE2\x94\x9C\xE2\x94\x80 ");
    }

    if is_dir {
        print_bytes(b"\xF0\x9F\x93\x81 ");
    } else {
        print_bytes(b"\xF0\x9F\x93\x84 ");
    }

    print_bytes(entry);
    print_bytes(b"\n");
}
