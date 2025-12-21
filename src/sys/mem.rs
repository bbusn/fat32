#[unsafe(no_mangle)]
pub fn memset(dst: *mut u8, val: i32, count: usize) -> *mut u8 {
    let mut i = 0;

    while i < count {
        unsafe {
            *dst.add(i) = val as u8;
        }
        i += 1;
    }

    dst
}

#[unsafe(no_mangle)]
pub fn memcmp(a: *const u8, b: *const u8, count: usize) -> i32 {
    let mut i = 0;
    while i < count {
        let av = unsafe { *a.add(i) };
        let bv = unsafe { *b.add(i) };
        if av != bv {
            return av as i32 - bv as i32;
        }
        i += 1;
    }
    0
}
