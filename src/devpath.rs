use crate::guid::Guid;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Protocol {
    pub r#type: u8,
    pub subtype: u8,
    pub length: [u8; 2],
}

impl Protocol {
    pub const GUID: Guid = Guid::new(
        0x09576E91,
        0x6D3F,
        0x11D2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    );
}

#[repr(C)]
pub struct Utils {
    pub get_devpath_size: extern "efiapi" fn(&Protocol) -> usize,
    pub duplicate_devpath: extern "efiapi" fn(&Protocol) -> *const u8,
    pub append_devpath: extern "efiapi" fn(&Protocol, &Protocol) -> *const u8,
    pub append_device_node: extern "efiapi" fn(&Protocol, &Protocol) -> *const u8,
    pub append_devpath_inst: extern "efiapi" fn(&Protocol, &Protocol) -> *const u8,
    pub get_next_devpath_inst: extern "efiapi" fn(*mut *mut Protocol, &mut usize) -> *const u8,
    pub is_devpath_multi_inst: extern "efiapi" fn(&Protocol) -> bool,
    pub create_node: extern "efiapi" fn(u8, u8, u16) -> *const u8,
}

impl Utils {
    pub const GUID: Guid = Guid::new(
        0x0379BE4E,
        0xD706,
        0x437D,
        [0xB0, 0x37, 0xED, 0xB8, 0x2F, 0xB7, 0x72, 0xA4],
    );
}
