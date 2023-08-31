use crate::sys;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec; 
use alloc::vec; 

#[derive(Clone, Copy)]

pub enum IO {Read, Write}

pub trait FileIO { 
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()>;
    fn close(&mut self);
    fn poll(&mut self, event: IO) -> bool;
}

pub fn dirname(pathname: &str) -> &str { 
    let n = pathname.len();
    let i = match pathname.rfind("/") { 
        Some(0) => 1,
        Some(i ) => i,
        None => n,
    };
    &pathname[..i]
} 

pub fn filename(pathname: &str) -> &str {
    let n = pathname.len();
    let i = match pathname.rfind('/') {
        Some(i) => i + 1,
        None => 0,
    };
    &pathname[i..n]
}

// Transform "hello.txt" into "/path/to/hello.txt"
pub fn realpath(pathname: &str) -> String {
    if pathname.starts_with('/') {
        pathname.into()
    } else {
        let dirname = sys::process::dir();
        let sep = if dirname.ends_with('/') { "" } else { "/" };
        format!("{}{}{}", dirname, sep, pathname)
    }
}

