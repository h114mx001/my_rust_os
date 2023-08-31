use crate::sys;
use crate::api::process::ExitCode;
use crate::sys::process::Process;

use alloc::vec;
use core::arch::asm; 

pub fn exit(code: ExitCode) -> ExitCode { 
    sys::process::exit();
    code
} 

pub fn sleep(seconds: f64) { 
    sys::time::sleep(seconds);
}

pub fn stop (code: usize) -> usize {
    match code { 
        0xcafe => { // Reboot
            unsafe {
                asm!(
                    "xor rax, rax",
                    "mov cr3, rax"
                );
            }
        }
        0xdead => { // Halt
            sys::process::exit();
            sys::acpi::shutdown();
        }
        _ => {
            debug!("STOP SYSCALL: Invalid code: {:#x}", code);
        }
    }
    0
}

// File System related 

