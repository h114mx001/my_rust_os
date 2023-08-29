mod block;
mod bitmap_block;
mod block_device;
mod device;
mod dir;
mod dir_entry;
mod file;
mod read_dir;
mod super_block;

use crate::sys;

pub use bitmap_block::BITMAP_SIZE;
pub use device::{Device, DeviceType};
pub use dir::Dir;
pub use dir_entry::FileInfo;
pub use file::{File, SeekFrom};
pub use block_device::{format_ata, format_mem, is_mounted, mount_ata, mount_mem, dismount};
pub use crate::api::fs::{dirname, filename, realpath, FileIO, IO};
pub use crate::sys::ata::BLOCK_SIZE;
pub use crate::sys::ata::Drive
use dir_entry::DirEntry;
use super_block::SuperBlock;

use alloc::string::{String, ToString};

pub const VERSION: u8 = 1;
