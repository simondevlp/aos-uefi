pub mod addr;
pub mod alloc;
mod descriptor;
pub use descriptor::*;

#[repr(C)]
pub enum Type {
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
