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
