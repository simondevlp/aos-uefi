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
        &Self,
        &mut *mut Self,
        *const u16,
        FileOpenMode,
        FileAttributes,
    ) -> Status,
    pub close: extern "efiapi" fn(&Self) -> Status,
    pub delete: extern "efiapi" fn(&Self) -> Status,
    pub read: extern "efiapi" fn(&Self, &mut usize, *mut u8) -> Status,
    pub write: extern "efiapi" fn(&Self, &mut usize, *mut u8) -> Status,
    pub get_pos: extern "efiapi" fn(&Self, &mut u64) -> Status,
    pub set_pos: extern "efiapi" fn(&Self, u64) -> Status,
    pub get_info: extern "efiapi" fn(&Self, &Guid, &mut usize, *mut u8) -> Status,
    pub set_info: extern "efiapi" fn(&Self, &Guid, usize, *const u8) -> Status,
    pub flush: extern "efiapi" fn(&Self) -> Status,
}
