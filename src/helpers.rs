// FAT32 uses little endian
pub fn u8_le_to_u16(bytes: &[u8]) -> u16 {
    (bytes[1] as u16) << 8 | (bytes[0] as u16)
}

pub fn u8_le_to_u32(bytes: &[u8]) -> u32 {
    (bytes[3] as u32) << 24 | (bytes[2] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[0] as u32)
}

pub fn u8_to_u32_le(bytes: &[u8]) -> u32 {
    (bytes[0] as u32) | (bytes[1] as u32) << 8 | (bytes[2] as u32) << 16 | (bytes[3] as u32) << 24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_le_to_u16_works() {
        let bytes = [0x34u8, 0x12u8]; /* 0x1234 -> 4660 */
        assert_eq!(u8_le_to_u16(&bytes), 0x1234);
    }

    #[test]
    fn u8_to_u32_le_works() {
        let bytes = [0x78u8, 0x56u8, 0x34u8, 0x12u8]; /* 0x12345678 */
        assert_eq!(u8_to_u32_le(&bytes), 0x12345678);
    }
}
