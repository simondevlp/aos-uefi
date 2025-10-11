use crate::{guid::Guid, services::time::Time};

#[repr(transparent)]
#[derive(Clone, Copy, Default)]
pub struct FileAttributes(pub u64);
impl FileAttributes {
    pub const READ_ONLY: Self = Self(0x1);
    pub const HIDDEN: Self = Self(0x2);
    pub const SYSTEM: Self = Self(0x4);
    pub const RESERVED: Self = Self(0x8);
    pub const DIRECTORY: Self = Self(0x10);
    pub const ARCHIVE: Self = Self(0x20);
    pub const VALID_ATTR: Self = Self(0x37);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FileInfo {
    pub size: u64,
    pub file_size: u64,
    pub phys_size: u64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub mod_time: Time,
    pub attr: FileAttributes,
    pub file_name: [u16; 256],
}

impl FileInfo {
    pub const GUID: Guid = Guid::new(
        0x09576e92,
        0x6d3f,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            size: Default::default(),
            file_size: Default::default(),
            phys_size: Default::default(),
            create_time: Default::default(),
            last_access_time: Default::default(),
            mod_time: Default::default(),
            attr: Default::default(),
            file_name: [0u16; 256],
        }
    }
}

#[repr(C)]
pub struct FileSystemInfo {
    pub size: u64,
    pub read_only: bool,
    pub vol_size: u64,
    pub free_space: u64,
    pub block_size: u32,
    pub vol_label: [u16; 256],
}

impl FileSystemInfo {
    pub const GUID: Guid = Guid::new(
        0x09576e93,
        0x6d3f,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

#[repr(C)]
pub struct VolumeLabel {
    pub vol_label: [u16; 256],
}

impl VolumeLabel {
    pub const GUID: Guid = Guid::new(
        0xdb47d7d3,
        0xfe81,
        0x11d3,
        [0x9a, 0x35, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
    );
}
