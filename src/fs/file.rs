use crate::{fs::fileinfo::FileAttributes, guid::Guid, status::Status};

#[repr(transparent)]
pub struct FileOpenMode(pub u64);
impl FileOpenMode {
    pub const READ: Self = Self(0x1);
    pub const WRITE: Self = Self(0x2);
    pub const CREATE: Self = Self(0x8000_0000_0000_0000);
}

#[repr(C)]
pub struct File {
    pub revision: u64,
    pub open: extern "efiapi" fn(
        this: &Self,
        new_handle: &mut *mut Self,
        file_name: *const u16,
        open_mode: FileOpenMode,
        attributes: FileAttributes,
    ) -> Status,
    pub close: extern "efiapi" fn(this: &Self) -> Status,
    pub delete: extern "efiapi" fn(this: &Self) -> Status,
    pub read: extern "efiapi" fn(this: &Self, buffer_size: &mut usize, buffer: *mut u8) -> Status,
    pub write: extern "efiapi" fn(this: &Self, buffer_size: &mut usize, buffer: *mut u8) -> Status,
    pub get_pos: extern "efiapi" fn(this: &Self, pos: &mut u64) -> Status,
    pub set_pos: extern "efiapi" fn(this: &Self, pos: u64) -> Status,
    pub get_info: extern "efiapi" fn(
        this: &Self,
        info_type: &Guid,
        buffer_size: &mut usize,
        buffer: *mut u8,
    ) -> Status,
    pub set_info: extern "efiapi" fn(
        this: &Self,
        info_type: &Guid,
        buffer_size: usize,
        buffer: *const u8,
    ) -> Status,
    pub flush: extern "efiapi" fn(this: &Self) -> Status,
    pub open_ex: extern "efiapi" fn(),
    pub read_ex: extern "efiapi" fn(),
    pub write_ex: extern "efiapi" fn(),
    pub flush_ex: extern "efiapi" fn(),
}
