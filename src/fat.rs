use crate::boot_sector::BootSector;
use crate::cli::{print, print_ls, reset_cli};
use crate::helpers::{u8_to_u32_le, u8_le_to_u16, to_lowercase_ascii};
use crate::sys::{print_bytes, read_at};

const FAT_MAX_SIZE: usize = 65536;
const CLUSTER_MAX_SIZE: usize = 65536;

fn fat_entry(fat_buf: &[u8], cluster: u32) -> u32 {
    let off = (cluster as usize) * 4;
    if off + 4 > fat_buf.len() {
        return 0x0FFFFFFF;
    }
    let v = u8_to_u32_le(&fat_buf[off..off + 4]);
    v & 0x0FFFFFFF
}

fn is_end_cluster(cluster: u32) -> bool {
    cluster >= 0x0FFFFFF8
}

fn read_fat_into(fd: usize, bs: &BootSector, fat_start: usize, fat_buf: &mut [u8]) -> isize {
    let bytes_per_sector = bs.bytes_per_sector as usize;
    let fat_size_bytes = bs.fat_size_sectors as usize * bytes_per_sector;
    let read_size = core::cmp::min(fat_buf.len(), fat_size_bytes);
    unsafe { read_at(fd, fat_buf.as_mut_ptr(), read_size, fat_start) }
}

fn read_cluster_into(
    fd: usize,
    bs: &BootSector,
    data_start: usize,
    cluster: u32,
    buf: &mut [u8],
) -> isize {
    let bytes_per_sector = bs.bytes_per_sector as usize;
    let sectors_per_cluster = bs.sectors_per_cluster as usize;
    let cluster_size = bytes_per_sector * sectors_per_cluster;
    if cluster < 2 {
        return -1;
    }
    if cluster_size > buf.len() {
        /* buffer provided is too small for a full cluster */
        return -1;
    }

    let read_size = cluster_size;

    let cluster_index = match (cluster as usize).checked_sub(2) {
        Some(v) => v,
        None => return -1,
    };

    let offset = match cluster_index.checked_mul(cluster_size) {
        Some(v) => match data_start.checked_add(v) {
            Some(o) => o,
            None => return -1,
        },
        None => return -1,
    };

    unsafe { read_at(fd, buf.as_mut_ptr(), read_size, offset) }
}

fn build_short_name(name: &[u8], ext: &[u8], out: &mut [u8]) -> usize {
    let mut idx = 0usize;

    /* Trim name trailing spaces */
    let mut name_end = name.len();
    while name_end > 0 && name[name_end - 1] == b' ' {
        name_end -= 1;
    }

    for j in 0..name_end {
        if idx >= out.len() { break; }
        out[idx] = name[j];
        idx += 1;
    }

    /* Trim ext */
    let mut ext_end = ext.len();
    while ext_end > 0 && ext[ext_end - 1] == b' ' {
        ext_end -= 1;
    }

    if ext_end > 0 {
        if idx < out.len() {
            out[idx] = b'.';
            idx += 1;
        }
        for j in 0..ext_end {
            if idx >= out.len() { break; }
            out[idx] = ext[j];
            idx += 1;
        }
    }

    idx
}

fn iterate_dir_entries<R, F>(
    fd: usize,
    bs: &BootSector,
    fat_start: usize,
    data_start: usize,
    start_cluster: u32,
    mut cb: F,
) -> Option<R>
where
    F: FnMut(&[u8], bool) -> Option<R>,
{
    let bytes_per_sector = bs.bytes_per_sector as usize;
    let sectors_per_cluster = bs.sectors_per_cluster as usize;
    let cluster_size = bytes_per_sector * sectors_per_cluster;

    if cluster_size == 0 || cluster_size > CLUSTER_MAX_SIZE {
        return None;
    }

    if start_cluster < 2 {
        return None;
    }

    let fat_size_bytes = bs.fat_size_sectors as usize * bytes_per_sector;
    let fat_buf_size = core::cmp::min(FAT_MAX_SIZE, fat_size_bytes);

    let mut fat_buf = [0u8; FAT_MAX_SIZE];

    let r = read_fat_into(fd, bs, fat_start, &mut fat_buf[..fat_buf_size]);
    if r < 0 || r as usize != fat_buf_size {
        return None;
    }

    let mut cluster_buf = [0u8; CLUSTER_MAX_SIZE];

    let mut cluster = start_cluster;

    while !is_end_cluster(cluster) {
        let rr = read_cluster_into(fd, bs, data_start, cluster, &mut cluster_buf[..cluster_size]);
        if rr < 0 || rr as usize != cluster_size {
            return None;
        }

        let entries = cluster_size / 32;

        for i in 0..entries {
            let off = i * 32;
            let entry = &cluster_buf[off..off + 32];

            let first = entry[0];
            if first == 0x00 {
                /* No more entries in this directory */
                return None;
            }
            if first == 0xE5 {
                /* Deleted */
                continue;
            }

            let attr = entry[11];
            if (attr & 0x0F) == 0x0F {
                /* LFN entry */
                continue;
            }
            if (attr & 0x08) != 0 {
                /* Volume id */
                continue;
            }

            let last = i == entries - 1;

            if let Some(res) = cb(entry, last) {
                return Some(res);
            }
        }

        /* Move to next cluster in chain */
        let next = fat_entry(&fat_buf[..fat_buf_size], cluster);
        if next == 0 || is_end_cluster(next) {
            break;
        }
        cluster = next;
    }

    None
}

pub fn list_root(fd: usize, bs: &BootSector, fat_start: usize, data_start: usize) {
    let _ = iterate_dir_entries::<(), _>(fd, bs, fat_start, data_start, bs.root_cluster, |entry, last| {
        let attr = entry[11];

        let name = &entry[0..8];
        let ext = &entry[8..11];

        let mut out = [0u8; 13];
        let name_len = build_short_name(name, ext, &mut out);
        let name_bytes = &out[..name_len];

        /* Convert to lowercase */
        let mut lower_name = [0u8; 13];
        let lower_len = to_lowercase_ascii(name_bytes, &mut lower_name);

        /* 0x10 = directory flag */
        let is_dir = (attr & 0x10) != 0;

        let indent_level = 0;

        print_ls(&lower_name[..lower_len], is_dir, last, indent_level);

        None
    });
}

pub fn change_directory(
    fd: usize,
    bs: &BootSector,
    fat_start: usize,
    data_start: usize,
    current_cluster: u32,
    dir_name: &[u8],
) -> Option<u32> {
    /* Prepare lowercase search bytes for the requested dir name */
    let mut lower_search = [0u8; 13];
    let search_len = to_lowercase_ascii(dir_name, &mut lower_search);

    /* Special cases: '.' -> stay, '..' -> parent */
    if search_len == 1 && lower_search[0] == b'.' {
        return Some(current_cluster);
    }

    /* If looking for parent, we still need to find the ".." entry in the current directory */
    if search_len == 2 && lower_search[0] == b'.' && lower_search[1] == b'.' {
        let found_parent = iterate_dir_entries::<u32, _>(fd, bs, fat_start, data_start, current_cluster, |entry, _last| {
            let name = &entry[0..8];
            let ext = &entry[8..11];

            let mut out = [0u8; 13];
            let name_len = build_short_name(name, ext, &mut out);
            let name_bytes = &out[..name_len];

            let mut lower_name = [0u8; 13];
            let lower_len = to_lowercase_ascii(name_bytes, &mut lower_name);

            if lower_len == search_len && &lower_name[..lower_len] == &lower_search[..search_len] {
                let cluster_low = u8_le_to_u16(&entry[26..28]) as u32;
                let cluster_high = u8_le_to_u16(&entry[20..22]) as u32;
                let target_cluster = (cluster_high << 16) | (cluster_low & 0xFFFF);
                return Some(target_cluster);
            }

            None
        });

        if let Some(mut target_cluster) = found_parent {
            if target_cluster < 2 {
                target_cluster = bs.root_cluster;
            }
            reset_cli();

            let _ = iterate_dir_entries::<(), _>(fd, bs, fat_start, data_start, target_cluster, |entry, last| {
                let attr = entry[11];

                let name = &entry[0..8];
                let ext = &entry[8..11];

                let mut out = [0u8; 13];
                let name_len = build_short_name(name, ext, &mut out);
                let name_bytes = &out[..name_len];

                /* Convert to lowercase */
                let mut lower_name = [0u8; 13];
                let lower_len = to_lowercase_ascii(name_bytes, &mut lower_name);

                /* 0x10 = directory flag */
                let is_dir = (attr & 0x10) != 0;

                let indent_level = 0;

                print_ls(&lower_name[..lower_len], is_dir, last, indent_level);

                None
            });

            return Some(target_cluster);
        }

        return None;
    }

    /* General case: find a matching subdirectory by name */
    let found = iterate_dir_entries::<u32, _>(fd, bs, fat_start, data_start, current_cluster, |entry, _last| {
        let attr = entry[11];
        /* 0x10 = directory flag */
        let is_dir = (attr & 0x10) != 0;
        if !is_dir {
            return None;
        }

        let name = &entry[0..8];
        let ext = &entry[8..11];

        let mut out = [0u8; 13];
        let name_len = build_short_name(name, ext, &mut out);
        let name_bytes = &out[..name_len];

        /* Convert to lowercase for comparison */
        let mut lower_name = [0u8; 13];
        let lower_len = to_lowercase_ascii(name_bytes, &mut lower_name);

        /* Compare names */
        if lower_len == search_len && &lower_name[..lower_len] == &lower_search[..search_len] {
            /* Found the directory, extract cluster number (low and high 16-bit parts) */
            let cluster_low = u8_le_to_u16(&entry[26..28]) as u32;
            let cluster_high = u8_le_to_u16(&entry[20..22]) as u32;
            let target_cluster = (cluster_high << 16) | (cluster_low & 0xFFFF);
            return Some(target_cluster);
        }

        None
    });

    if let Some(target_cluster) = found {
        /* Clear the screen and print the directory contents like `list_root` */
        reset_cli();

        let _ = iterate_dir_entries::<(), _>(fd, bs, fat_start, data_start, target_cluster, |entry, last| {
            let attr = entry[11];

            let name = &entry[0..8];
            let ext = &entry[8..11];

            let mut out = [0u8; 13];
            let name_len = build_short_name(name, ext, &mut out);
            let name_bytes = &out[..name_len];

            /* Convert to lowercase */
            let mut lower_name = [0u8; 13];
            let lower_len = to_lowercase_ascii(name_bytes, &mut lower_name);

            /* 0x10 = directory flag */
            let is_dir = (attr & 0x10) != 0;

            let indent_level = 0;

            print_ls(&lower_name[..lower_len], is_dir, last, indent_level);

            None
        });

        return Some(target_cluster);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fat_entry_valid() {
        let mut fat_buf = [0u8; 16];
        /* Store a valid cluster value (0x12345678) at offset 0 */
        fat_buf[0] = 0x78;
        fat_buf[1] = 0x56;
        fat_buf[2] = 0x34;
        fat_buf[3] = 0x12;

        let result = fat_entry(&fat_buf, 0);
        assert_eq!(result, 0x12345678);
    }

    #[test]
    fn test_fat_entry_with_mask() {
        let mut fat_buf = [0u8; 16];
        /* Store 0xF2345678 (should be masked to 0x02345678) */
        fat_buf[0] = 0x78;
        fat_buf[1] = 0x56;
        fat_buf[2] = 0x34;
        fat_buf[3] = 0xF2;

        let result = fat_entry(&fat_buf, 0);
        assert_eq!(result, 0x02345678);
    }

    #[test]
    fn test_fat_entry_multiple_clusters() {
        let mut fat_buf = [0u8; 24];
        /* Cluster 0 */
        fat_buf[0] = 0x00;
        fat_buf[1] = 0x00;
        fat_buf[2] = 0x00;
        fat_buf[3] = 0x00;
        /* Cluster 1 */
        fat_buf[4] = 0x22;
        fat_buf[5] = 0x11;
        fat_buf[6] = 0x00;
        fat_buf[7] = 0x00;
        /* Cluster 2 */
        fat_buf[8] = 0x44;
        fat_buf[9] = 0x33;
        fat_buf[10] = 0x00;
        fat_buf[11] = 0x00;

        assert_eq!(fat_entry(&fat_buf, 0), 0x00000000);
        assert_eq!(fat_entry(&fat_buf, 1), 0x00001122);
        assert_eq!(fat_entry(&fat_buf, 2), 0x00003344);
    }

    #[test]
    fn test_fat_entry_out_of_bounds() {
        let fat_buf = [0u8; 8];
        /* Try to read cluster that would be out of bounds */
        let result = fat_entry(&fat_buf, 10);
        assert_eq!(result, 0x0FFFFFFF);
    }

    #[test]
    fn test_is_end_cluster() {
        assert!(!is_end_cluster(0x0FFFF000));
        assert!(!is_end_cluster(0x0FFFFFF0));
        assert!(!is_end_cluster(0x0FFFFFF7));
        assert!(is_end_cluster(0x0FFFFFF8));
        assert!(is_end_cluster(0x0FFFFFF9));
        assert!(is_end_cluster(0x0FFFFFFA));
        assert!(is_end_cluster(0x0FFFFFFB));
        assert!(is_end_cluster(0x0FFFFFFC));
        assert!(is_end_cluster(0x0FFFFFFD));
        assert!(is_end_cluster(0x0FFFFFFE));
        assert!(is_end_cluster(0x0FFFFFFF));
    }

    #[test]
    fn test_is_end_cluster_boundary() {
        assert!(!is_end_cluster(0x0FFFFFF7));
        assert!(is_end_cluster(0x0FFFFFF8));
    }

    #[test]
    fn test_fat_entry_zero_value() {
        let fat_buf = [0u8; 16];
        let result = fat_entry(&fat_buf, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fat_entry_all_ones() {
        let mut fat_buf = [0xFFu8; 16];
        let result = fat_entry(&fat_buf, 0);
        /* 0xFFFFFFFF & 0x0FFFFFFF = 0x0FFFFFFF */
        assert_eq!(result, 0x0FFFFFFF);
    }
}
