use crate::sys::consts::{AT_FDCWD, RDONLY_0, SEEK_CUR, SEEK_END, SEEK_SET, STDOUT_FILENO};

pub mod syscalls {
    pub const EXIT: usize = 60;
    pub const OPEN_AT: usize = 257;
    pub const READ: usize = 0;
    pub const CLOSE: usize = 3;
    pub const WRITE: usize = 1;
    pub const LSEEK: usize = 8;
}

/* __________ Syscalls __________ */
#[inline(always)]
pub fn syscall_3(n: usize, a0: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a0,
            in("rsi") a1,
            in("rdx") a2,
            lateout("rax") ret,
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
            "syscall",
            in("rax") n,
            in("rdi") a0,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret
}

/* __________ Helpers __________ */
pub fn exit(code: usize) {
    syscall_1(syscalls::EXIT, code);
}

pub fn lseek(fd: usize, offset: usize, whence: usize) -> isize {
    syscall_3(syscalls::LSEEK, fd, offset, whence)
}

pub unsafe fn read_at(fd: usize, buffer: *mut u8, len: usize, offset: usize) -> isize {
    let cur = lseek(fd, 0, SEEK_CUR);
    if cur < 0 {
        return cur;
    }

    if lseek(fd, offset, SEEK_SET) < 0 {
        return -1;
    }

    let r = read(fd, buffer, len);

    lseek(fd, cur as usize, SEEK_SET);

    r
}

pub fn open(path: *const u8) -> isize {
    syscall_3(
        syscalls::OPEN_AT,
        AT_FDCWD as usize,
        path as usize,
        RDONLY_0,
    )
}

pub fn read(fd: usize, buffer: *mut u8, len: usize) -> isize {
    syscall_3(syscalls::READ, fd, buffer as usize, len)
}

pub fn close(fd: usize) {
    syscall_1(syscalls::CLOSE, fd);
}

pub fn write(fd: usize, val: *const u8, len: usize) {
    syscall_3(syscalls::WRITE, fd, val as usize, len);
}

pub fn print_bytes(val: &[u8]) {
    write(STDOUT_FILENO, val.as_ptr(), val.len());
}
