use crate::Handle;

#[repr(C)]
pub enum InterfaceType {
    NativeInterface,
}

#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

#[repr(transparent)]
pub struct OpenProtocolAttributes(pub u32);

impl OpenProtocolAttributes {
    pub const BY_HANDLE_PROTOCOL: Self = Self(0x1);
    pub const GET_PROTOCOL: Self = Self(0x2);
    pub const TEST_PROTOCOL: Self = Self(0x4);
    pub const BY_CHILD_CONTROLLER: Self = Self(0x8);
    pub const BY_DRIVER: Self = Self(0x10);
    pub const EXCLUSIVE: Self = Self(0x20);
}

#[repr(C)]
pub struct OpenProtocolInfoEntry {
    pub agent: Handle,
    pub controller: Handle,
    pub attributes: OpenProtocolAttributes,
    pub open_count: u32,
}
