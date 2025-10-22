use crate::{guid::Guid, memory, status::Status};

pub mod blt;

#[repr(C)]
pub struct PixelBitmask {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub reserved: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum PixelFormat {
    RGBRsv8bit,
    BGRRsv8bit,
    BitMask,
    BltOnly,
    Max,
}

impl PixelFormat {
    pub fn pitch(&self) -> usize {
        match self {
            Self::RGBRsv8bit => 4,
            Self::BGRRsv8bit => 4,
            Self::BitMask => 16,
            _ => 0,
        }
    }
}

#[repr(C)]
pub struct ModeInfo {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub verticle_resolution: u32,
    pub pixel_format: PixelFormat,
    pub pixel_info: PixelBitmask,
    pub pixels_per_scanline: u32,
}

#[repr(C)]
pub struct Mode {
    pub max: u32,
    pub mode: u32,
    pub info: &'static ModeInfo,
    pub info_size: usize,
    pub base: memory::addr::Physical,
    pub size: usize,
}

#[repr(C)]
pub struct Protocol {
    pub query_mode: extern "C" fn(&Self, u32, &mut usize, &mut *mut u8) -> Status,
    pub set_mode: extern "C" fn(&Self, u32) -> Status,
    pub blt: extern "C" fn(
        &Self,
        *mut blt::Pixel,
        blt::Operation,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) -> Status,
    pub mode: &'static Mode,
}

impl Protocol {
    pub const GUID: Guid = Guid::new(
        0x9042A9DE,
        0x23DC,
        0x4A38,
        [0x96, 0xFB, 0x7A, 0xDE, 0xD0, 0x80, 0x51, 0x6A],
    );
}
