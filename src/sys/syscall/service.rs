use crate::api::fs::{FileIO, IO};
use crate::api::process::ExitCode;
use crate::sys;
use crate::sys::fs::FileInfo;
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

pub fn stop(code: usize) -> usize {
    match code {
        0xcafe => {
            // Reboot
            unsafe {
                asm!("xor rax, rax", "mov cr3, rax");
            }
        }
        0xdead => {
            // Halt
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

pub fn delete(path: &str) -> isize {
    if sys::fs::delete(path).is_ok() {
        0
    } else {
        -1
    }
}

pub fn info(path: &str, info: &mut FileInfo) -> isize {
    let path = match sys::fs::canonicalize(path) {
        Ok(path) => path,
        Err(_) => return -1,
    };
    if let Some(res) = sys::fs::info(&path) {
        *info = res;
        0
    } else {
        -1
    }
}

pub fn open(path: &str, flags: usize) -> isize {
    let path = match sys::fs::canonicalize(path) {
        Ok(path) => path,
        Err(_) => return -1,
    };
    if let Some(resource) = sys::fs::open(&path, flags) {
        if let Ok(handle) = sys::process::create_handle(resource) {
            return handle as isize;
        }
    }
    -1
}

pub fn dup(old_handle: usize, new_handle: usize) -> isize {
    if let Some(file) = sys::process::handle(old_handle) {
        sys::process::update_handle(new_handle, *file);
        return new_handle as isize;
    }
    -1
}

pub fn read(handle: usize, buf: &mut [u8]) -> isize {
    if let Some(mut file) = sys::process::handle(handle) {
        if let Ok(bytes) = file.read(buf) {
            sys::process::update_handle(handle, *file);
            return bytes as isize;
        }
    }
    -1
}

pub fn write(handle: usize, buf: &mut [u8]) -> isize {
    if let Some(mut file) = sys::process::handle(handle) {
        if let Ok(bytes) = file.write(buf) {
            sys::process::update_handle(handle, *file);
            return bytes as isize;
        }
    }
    -1
}

pub fn close(handle: usize) {
    if let Some(mut file) = sys::process::handle(handle) {
        file.close();
        sys::process::delete_handle(handle);
    }
}

pub fn spawn(path: &str, args_ptr: usize, args_len: usize) -> ExitCode {
    let path = match sys::fs::canonicalize(path) {
        Ok(path) => path,
        Err(_) => return ExitCode::OpenError,
    };
    if let Some(mut file) = sys::fs::File::open(&path) {
        let mut buf = vec![0; file.size()];
        if let Ok(bytes) = file.read(&mut buf) {
            buf.resize(bytes, 0);
            if let Err(code) = Process::spawn(&buf, args_ptr, args_len) {
                code
            } else {
                ExitCode::Success
            }
        } else {
            ExitCode::ReadError
        }
    } else {
        ExitCode::OpenError
    }
}

pub fn poll(list: &[(usize, IO)]) -> isize {
    for (i, (handle, event)) in list.iter().enumerate() {
        if let Some(mut file) = sys::process::handle(*handle) {
            if file.poll(*event) {
                return i as isize;
            }
        }
    }
    -1
}
