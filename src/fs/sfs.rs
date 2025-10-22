use crate::{fs::file, guid::Guid, status::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open_volume: extern "efiapi" fn(this: &Self, root: &mut *mut file::Protocol) -> Status,
}

impl Protocol {
    pub const GUID: Guid = Guid::new(
        0x0964e5b22,
        0x6459,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
