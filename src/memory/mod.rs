pub mod descriptor;

#[repr(C)]
pub enum AllocType {
    AnyPages,
    MaxAddress,
    Address,
    Max,
}

#[repr(C)]
pub enum MemoryType {
    Reserved,
    LoaderCode,
    LoaderData,
    BootSrvCode,
    BootSrvData,
    RuntimeSrvCode,
    RuntimeSrvData,
    Conventional,
    Unusable,
    ACPIReclaim,
    ACPINVS,
    MappedIO,
    MappedIOPortSpace,
    PalCode,
    Persistent,
    Unaccepted,
    Max,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

#[repr(transparent)]
pub struct VirtualAddress(pub u64);
