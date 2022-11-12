#![no_main]
#![no_std]
mod lang_items;

// #[macro_use]
// mod console;

core::arch::global_asm!(include_str!("entry.asm"));

#[macro_use]
mod sbi;
use crate::sbi::shutdown;

const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

// wrapper of sys_write
const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
  syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}


#[no_mangle]
pub fn rust_main() -> ! {
    // print!("Hello, ");
    // println!("world!");
    // sys_exit(9);
    shutdown();
}


// fn main() {
    // println!("Hello, world!");
// }
