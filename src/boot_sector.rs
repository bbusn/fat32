use crate::helpers::{ u8_le_to_u16, u8_le_to_u32};

pub struct BootSector {
	bytes_per_sector: u16,
	sectors_per_cluster: u8,
	reserved_sectors_count: u16,
	fat_count: u8,
    	fat_size_sectors: u32,
	root_cluster: u32,
}

pub fn verify_boot_sector_signature(bs: &[u8; 512]) -> bool {
    return bs[510] == 0x55 && bs[511] == 0xAA
}

pub fn parse_boot_sector(bs: &[u8; 512]) -> BootSector {
	BootSector {
		bytes_per_sector: u8_le_to_u16(&bs[11..13]),
                sectors_per_cluster: bs[13],
                reserved_sectors_count: u8_le_to_u16(&bs[14..16]),
                fat_count: bs[16],
		fat_size_sectors: u8_le_to_u32(&bs[36..40]),
                root_cluster: u8_le_to_u32(&bs[44..48]),
	}
}
