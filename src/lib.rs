#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;

// Test Framework 
pub trait Testable {
    fn run (&self) -> (); 
}

impl<T> Testable for T 
where 
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]")
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {};
}

pub fn test_runner(tests: &[&dyn Testable]) { 
    println!("Running {} tests", tests.len());
    for test in tests{
        test.run();
    }

    // Use for QEMU exit on debug exit code 
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn sample_test(){
    assert_eq!(1, 1);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, 
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe { 
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}