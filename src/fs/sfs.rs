use crate::{fs::file, status::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open_volume: extern "efiapi" fn(this: &Self, root: &mut *mut file::Protocol) -> Status,
}
