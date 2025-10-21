#[repr(C)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

#[repr(C)]
pub enum Operation {
    VideoFill,
    BltVideoToBltBuffer,
    BltBufferToVideo,
    BltVideoToVideo,
    Max,
}
