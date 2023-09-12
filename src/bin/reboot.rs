#![no_std]
#![no_main]

use rust_os::api::syscall;
use rust_os::entry_point;

entry_point!(main);

fn main(_args: &[&str]) {
    syscall::write(1, b"\x1b[93m"); // Yellow
    syscall::write(1, b"Oh, mamma mia, mamma mia.\n");
    syscall::write(1, b"Mamma mia, let me go.\n");
    syscall::write(1, b"\x1b[0m"); // Reset
    syscall::sleep(0.5);
    syscall::reboot();
    loop { syscall::sleep(1.0) }
}
