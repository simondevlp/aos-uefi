use crate::{
    Handle, boot::BootServices, config::ConfigTable, console::output::TextOutput,
    runtime::RuntimeServices, table::TableHeader,
};

#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub cin_handle: Handle,
    pub cin: usize,
    pub cout_handle: Handle,
    pub cout: &'static TextOutput,
    pub stderr_handle: Handle,
    pub stderr: &'static TextOutput,
    pub runtime_srv: &'static RuntimeServices,
    pub boot_srv: &'static BootServices,
    pub num_tab_entries: usize,
    pub config_tab: *const ConfigTable,
}
