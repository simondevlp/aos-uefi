use crate::{
    Handle,
    boot::BootServices,
    config::ConfigTable,
    console::{input, output},
    runtime::RuntimeServices,
    table::TableHeader,
};

#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub cin_handle: Handle,
    pub cin: &'static input::Protocol,
    pub cout_handle: Handle,
    pub cout: &'static output::Protocol,
    pub stderr_handle: Handle,
    pub stderr: &'static output::Protocol,
    pub runtime_srv: &'static RuntimeServices,
    pub boot_srv: &'static BootServices,
    pub num_tab_entries: usize,
    pub config_tab: *const ConfigTable,
}
