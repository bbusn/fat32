static RDONLY_0: usize = 0;
static AT_FDCWD: isize = -100;
static STDOUT_FILENO: usize = 1;
static HEX : &[u8; 16] = b"0123456789ABCDEF";

#[cfg(target_arch = "x86_64")]
mod sys_numbers {
    static EXIT: usize = 60;
    static OPEN_AT: usize = 257;
    static READ: usize = 0;
    static CLOSE: usize = 3;
    static WRITE: usize = 1;
}

/* Adding aarch64 syscalls because i'm on mac */
#[cfg(target_arch = "aarch64")]
mod sys_numbers {
    static EXIT: usize = 93;
    static OPEN_AT: usize = 56;
    static READ: usize = 63;
    static CLOSE: usize = 57;
    static WRITE: usize = 64;
}

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

        return ret;
    }
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

        return ret;
    }
}

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
    let ptr = val.as_ptr();
    let len = N;

    write(STDOUT_FILENO, ptr, len);
}
