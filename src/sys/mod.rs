pub mod statics;

use statics::{ HEX };

/* __________ aarch64 __________ */
#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

/* __________ x86_64 __________ */
#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;


/* __________ Helpers __________ */
pub fn print(val: &str) {
    print_bytes(val.as_bytes());
    print_bytes(b"\n");
}

// Convert a byte to hex
pub fn byte_to_hex(byte: u8) -> [u8; 3] {
    [
        HEX[(byte >> 4) as usize],
        HEX[(byte & 0x0F) as usize],
        b' ',
    ]
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
