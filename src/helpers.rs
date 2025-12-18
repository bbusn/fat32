// FAT32 uses little endian
pub fn u8_le_to_u16(bytes: &[u8]) -> u16 {
	(bytes[1] as u16) << 8 | (bytes[0] as u16)
}

pub fn u8_le_to_u32(bytes: &[u8]) -> u32 {
	(bytes[3] as u32) << 24 |
	(bytes[2] as u32) << 16 |	
        (bytes[1] as u32) << 8 |
        (bytes[0] as u32)
}
