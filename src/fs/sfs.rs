use crate::{fs::file::File, status::Status};

#[repr(C)]
pub struct SimpleFileSystem {
    pub revision: u64,
    pub open_volume: extern "efiapi" fn(this: &Self, root: &mut *mut File) -> Status,
}
