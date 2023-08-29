use super::bitmap_block::BitmapBlock;
use super::block_device::BlockDeviceIO;

use core::convert::TryInto;

const DATA_OFFSET: usize = 4; 

#[derive(Clone)]
pub struct Block { 
    addr: u32, 
    buf: [u8; super::BLOCK_SIZE]
}

// Block structure: 
// 0..4: Next block addr 
// 4..512: block data 

impl Block {
    pub fn new(addr: u32) -> Self { 
        let buf = [0; super::BLOCK_SIZE];
        Self {addr, buf}
    }

    pub fn alloc() -> Option<Self> {
        !unimplemented!()
    }

    pub fn read(addr: u32) -> Self {
        let mut buf = [0; super::BLOCK_SIZE];
        if let Some(ref mut block_device) = *super::block_device::BLOCK_DEVICE.lock() {
            if block_device.read(addr, &mut buf).
        }
    }
}