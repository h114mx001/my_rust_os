#![no_std]
#![no_main]

use rust_os::api::syscall;
use rust_os::entry_point;

entry_point!(main);

fn main(_args: &[&str]) {
    syscall::write(1, b"\x1b[93m"); // Yellow
    syscall::write(1, b"Goodbye everybody, I've got to go.\n");
    syscall::write(1, b"Got to leave you all behind and face the truth.\n");
    syscall::write(1, b"\x1b[0m"); // Reset
    syscall::write(1, b"\n");
    syscall::sleep(0.5);
    syscall::halt();
    loop { syscall::sleep(1.0) }
}
