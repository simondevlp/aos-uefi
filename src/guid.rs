#[repr(C)]
pub struct Guid {
    d0: u32,
    d1: u16,
    d2: u16,
    d3: [u8; 8],
}

impl Guid {
    pub const fn new(d0: u32, d1: u16, d2: u16, d3: [u8; 8]) -> Self {
        Self { d0, d1, d2, d3 }
    }
}
