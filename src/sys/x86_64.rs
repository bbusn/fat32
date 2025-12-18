use crate::sys::statics::{ AT_FDCWD, RDONLY_0, STDOUT_FILENO };

pub mod syscalls {
    pub const EXIT: usize = 60;
    pub const OPEN_AT: usize = 257;
    pub const READ: usize = 0;
    pub const CLOSE: usize = 3;
    pub const WRITE: usize = 1;
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
