use crate::boot_sector::BootSector;
use crate::cli::{print, print_ls};
use crate::helpers::u8_to_u32_le;
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
    let read_size = core::cmp::min(buf.len(), cluster_size);
    let offset = data_start + ((cluster as usize - 2) * cluster_size);
    unsafe { read_at(fd, buf.as_mut_ptr(), read_size, offset) }
}

pub fn list_root(fd: usize, bs: &BootSector, fat_start: usize, data_start: usize) {
    let bytes_per_sector = bs.bytes_per_sector as usize;
    let sectors_per_cluster = bs.sectors_per_cluster as usize;
    let cluster_size = bytes_per_sector * sectors_per_cluster;

    let fat_size_bytes = bs.fat_size_sectors as usize * bytes_per_sector;
    let fat_buf_size = core::cmp::min(FAT_MAX_SIZE, fat_size_bytes);

    let mut fat_buf = [0u8; FAT_MAX_SIZE];

    let r = read_fat_into(fd, bs, fat_start, &mut fat_buf[..fat_buf_size]);
    if r < 0 || r as usize != fat_buf_size {
        print("Failed to read FAT for ls");
        return;
    }

    let mut cluster_buf = [0u8; CLUSTER_MAX_SIZE];

    let mut cluster = bs.root_cluster;

    while !is_end_cluster(cluster) {
        let rr = read_cluster_into(
            fd,
            bs,
            data_start,
            cluster,
            &mut cluster_buf[..cluster_size],
        );
        if rr < 0 || rr as usize != cluster_size {
            print("Failed to read cluster for ls");
            return;
        }

        let entries = cluster_size / 32;

        for i in 0..entries {
            let off = i * 32;
            let entry = &cluster_buf[off..off + 32];

            let first = entry[0];
            if first == 0x00 {
                /* No more entries in this directory */
                return;
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

            let name = &entry[0..8];
            let ext = &entry[8..11];

            let mut out = [0u8; 13];
            let mut idx = 0usize;

            /* Trim name trailing spaces */
            let mut name_end = 8usize;
            while name_end > 0 && name[name_end - 1] == b' ' {
                name_end -= 1;
            }

            for j in 0..name_end {
                out[idx] = name[j];
                idx += 1;
            }

            /* Trim ext */
            let mut ext_end = 3usize;
            while ext_end > 0 && ext[ext_end - 1] == b' ' {
                ext_end -= 1;
            }

            if ext_end > 0 {
                out[idx] = b'.';
                idx += 1;
                for j in 0..ext_end {
                    out[idx] = ext[j];
                    idx += 1;
                }
            }

            out[idx] = b'\n';
            idx += 1;

            /* We remove the \n */
            let name_bytes = &out[..idx - 1];

            /* Convert to lowercase */
            let mut lower_name = [0u8; 13];
            for i in 0..name_bytes.len() {
                let c = name_bytes[i];
                if c >= b'A' && c <= b'Z' {
                    lower_name[i] = c + 32; // a-z
                } else {
                    lower_name[i] = c;
                }
            }

            /* 0x10 = directory flag */
            let is_dir = (attr & 0x10) != 0;

            let last = i == entries - 1;
            let indent_level = 0;

            print_ls(&lower_name[..name_bytes.len()], is_dir, last, indent_level);
        }

        /* Move to next cluster in chain */
        let next = fat_entry(&fat_buf[..fat_buf_size], cluster);
        if next == 0 || is_end_cluster(next) {
            break;
        }
        cluster = next;
    }
}
