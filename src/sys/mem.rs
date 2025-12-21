#[unsafe(no_mangle)]
pub fn memset(dst: *mut u8, val: i32, count: usize) -> *mut u8 {
    let mut i = 0;

    while i < count {
        /// Safety: writing via raw pointer arithmetic.
        ///
        /// `dst` must be valid for writes of `count` bytes. The caller is
        /// responsible for ensuring the pointer points to a writable region.
        // SAFETY: we write at `dst.add(i)` which must be valid for the current index.
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
        /// Safety: dereferencing raw pointers for comparison.
        ///
        /// Both `a` and `b` must be valid for reads of `count` bytes and non-null.
        // SAFETY: `a.add(i)` and `b.add(i)` must be valid for the current index.
        let av = unsafe { *a.add(i) };
        let bv = unsafe { *b.add(i) };
        if av != bv {
            return av as i32 - bv as i32;
        }
        i += 1;
    }
    0
}
