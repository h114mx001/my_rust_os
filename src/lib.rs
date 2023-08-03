#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use x86_64::instructions::hlt;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;


pub fn init(){
    gdt::init();
    interrupts::init_idt();
    unsafe { 
        interrupts::PICS.lock().initialize();
    };
    x86_64::instructions::interrupts::enable(); 
}

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

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
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
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}