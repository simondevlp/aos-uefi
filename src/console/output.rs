use crate::{guid::Guid, status::Status};

#[repr(C)]
pub struct Protocol {
    pub reset: extern "efiapi" fn(this: &Self, extended_verification: bool) -> Status,
    pub output: extern "efiapi" fn(this: &Self, string: *const u16) -> Status,
    pub test: usize,
    pub query_mode: usize,
    pub set_mode: usize,
    pub set_attr: extern "efiapi" fn(this: &Self, attr: usize) -> Status,
    pub clear: extern "efiapi" fn(this: &Self) -> Status,
    pub set_cursor_pos: extern "efiapi" fn(this: &Self, col: usize, row: usize) -> Status,
    pub enable_cursor: extern "efiapi" fn(this: &Self, visible: bool) -> Status,
    pub mode: &'static Mode,
}

impl Protocol {
    pub const GUID: Guid = Guid::new(
        0x387477c2,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

#[repr(C)]
pub struct Mode {
    max_mode: i32,
    mode: i32,
    attr: i32,
    cursor_col: i32,
    cursor_row: i32,
    cursor_vis: bool,
}

#[repr(transparent)]
pub struct Attribute(pub i32);

impl Attribute {
    pub const BLACK: Self = Self(0x0);
    pub const BLUE: Self = Self(0x1);
    pub const GREEN: Self = Self(0x2);
    pub const CYAN: Self = Self(0x3);
    pub const RED: Self = Self(0x4);
    pub const MAGENTA: Self = Self(0x5);
    pub const BROWN: Self = Self(0x6);
    pub const LIGHT_GRAY: Self = Self(0x7);
    pub const DARK_GRAY: Self = Self(0x8);
    pub const LIGHT_BLUE: Self = Self(0x9);
    pub const LIGHT_GREEN: Self = Self(0xa);
    pub const LIGHT_CYAN: Self = Self(0xb);
    pub const LIGHT_RED: Self = Self(0xc);
    pub const LIGHT_MAGENTA: Self = Self(0xd);
    pub const YELLOW: Self = Self(0xe);
    pub const WHITE: Self = Self(0xf);
    pub const BG_BLACK: Self = Self(0x00);
    pub const BG_BLUE: Self = Self(0x10);
    pub const BG_GREEN: Self = Self(0x20);
    pub const BG_CYAN: Self = Self(0x30);
    pub const BG_RED: Self = Self(0x40);
    pub const BG_MAGENTA: Self = Self(0x50);
    pub const BG_BROWN: Self = Self(0x60);
    pub const BG_LIGHT_GRAY: Self = Self(0x70);
}
