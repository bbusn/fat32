use crate::syscalls::common::*;

pub mod sys_numbers {
    pub const EXIT: usize = 93;
    pub const OPEN_AT: usize = 56;
    pub const READ: usize = 63;
    pub const CLOSE: usize = 57;
    pub const WRITE: usize = 64;
}

/* __________ Syscalls __________ */
#[inline(always)]
pub fn syscall_3(n: usize, a0: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") n,
            in("x0") a0,
            in("x1") a1,
            in("x2") a2,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn syscall_1(n: usize, a0: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") n,
            in("x0") a0,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

/* __________ Helpers __________ */
pub fn exit(code: usize) {
    syscall_1(sys_numbers::EXIT, code);
}

pub fn open(path: *const u8) -> isize {
    syscall_3(
        sys_numbers::OPEN_AT,
        AT_FDCWD as usize,
        path as usize,
        RDONLY_0,
    )
}

pub fn read(fd: usize, buffer: *mut u8, len: usize) -> isize {
    syscall_3(sys_numbers::READ, fd, buffer as usize, len)
}

pub fn close(fd: usize) {
    syscall_1(sys_numbers::CLOSE, fd);
}

pub fn write(fd: usize, val: *const u8, len: usize) {
    syscall_3(sys_numbers::WRITE, fd, val as usize, len);
}

pub fn print<const N: usize>(val: &[u8; N]) {
    write(STDOUT_FILENO, val.as_ptr(), N);
}

pub fn print_hex(byte: u8) {
    let buf = [HEX[(byte >> 4) as usize], HEX[(byte & 0x0F) as usize], b' '];
    print(&buf);
}
