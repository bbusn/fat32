use crate::helpers::{u8_le_to_u16, u8_to_u32_le};

pub struct BootSector {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors_count: u16,
    pub fats_count: u8,
    pub fat_size_sectors: u32,
    pub root_cluster: u32,
}

pub fn verify_boot_sector_signature(bs: &[u8; 512]) -> bool {
    return bs[510] == 0x55 && bs[511] == 0xAA;
}

pub fn parse_boot_sector(bs: &[u8; 512]) -> BootSector {
    BootSector {
        bytes_per_sector: u8_le_to_u16(&bs[11..13]),
        sectors_per_cluster: bs[13],
        reserved_sectors_count: u8_le_to_u16(&bs[14..16]),
        fats_count: bs[16],
        fat_size_sectors: u8_to_u32_le(&bs[36..40]),
        root_cluster: u8_to_u32_le(&bs[44..48]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_signature_true() {
        let mut bs = [0u8; 512];
        bs[510] = 0x55;
        bs[511] = 0xAA;

        assert!(verify_boot_sector_signature(&bs));
    }

    #[test]
    fn parse_returns_expected_fields() {
        let mut bs = [0u8; 512];

        /* Bytes_per_sector = 512 -> 0x0200 (little endian) */
        bs[11] = 0x00;
        bs[12] = 0x02;

        /* Sectors_per_cluster */
        bs[13] = 8;

        /* Reserved_sectors_count = 32 -> 0x0020 */
        bs[14] = 0x20;
        bs[15] = 0x00;

        /* Fats_count */
        bs[16] = 2;

        /* Fat_size_sectors = 12345 -> little endian */
        bs[36] = 0x39;
        bs[37] = 0x30;
        bs[38] = 0x00;
        bs[39] = 0x00;

        /* Root_cluster = 2 */
        bs[44] = 0x02;
        bs[45] = 0x00;
        bs[46] = 0x00;
        bs[47] = 0x00;

        let parsed = parse_boot_sector(&bs);

        assert_eq!(parsed.bytes_per_sector, 512);
        assert_eq!(parsed.sectors_per_cluster, 8);
        assert_eq!(parsed.reserved_sectors_count, 32);
        assert_eq!(parsed.fats_count, 2);
        assert_eq!(parsed.fat_size_sectors, 12345);
        assert_eq!(parsed.root_cluster, 2);
    }
}
