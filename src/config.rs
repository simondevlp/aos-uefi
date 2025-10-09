use crate::guid::Guid;

#[repr(C)]
pub struct ConfigTable {
    vendor_guid: Guid,
    vendor_tab: usize,
}
