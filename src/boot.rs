use crate::{
    Handle,
    guid::Guid,
    memory::{AllocType, MemoryType, PhysicalAddress, descriptor::MemoryDescriptor},
    services::event::Event,
    status::Status,
    table::TableHeader,
};

#[repr(C)]
pub struct BootServices {
    hdr: TableHeader,

    // task priority
    raise_tpl: extern "efiapi" fn(),
    restore_tpl: extern "efiapi" fn(),

    // memory (correct UEFI specification order)
    pub alloc_pages:
        extern "efiapi" fn(AllocType, MemoryType, usize, &mut PhysicalAddress) -> Status,
    pub free_pages: extern "efiapi" fn(&mut PhysicalAddress, usize) -> Status,
    pub get_mem_map:
        extern "efiapi" fn(&mut usize, usize, &mut usize, &mut usize, &mut u32) -> Status,
    pub alloc_pool: extern "efiapi" fn(MemoryType, usize, &mut usize) -> Status,
    pub free_pool: extern "efiapi" fn(usize) -> Status,

    // event & timer
    create_event: extern "efiapi" fn(),
    set_timer: extern "efiapi" fn(),
    wait_for_event: extern "efiapi" fn(usize, *const Event, &mut usize),
    signal_event: extern "efiapi" fn(),
    close_event: extern "efiapi" fn(),
    check_event: extern "efiapi" fn(),

    // protocol handler
    install_protocol: extern "efiapi" fn(),
    reinstall_protocol: extern "efiapi" fn(),
    uninstall_protocol: extern "efiapi" fn(),
    handle_protocol: extern "efiapi" fn(),
    reserved: usize,
    register_protocol_notify: extern "efiapi" fn(),
    locate_handle: extern "efiapi" fn(),
    locate_devpath: extern "efiapi" fn(),
    install_configtab: extern "efiapi" fn(),

    // image
    load_image: extern "efiapi" fn(),
    start_image: extern "efiapi" fn(),
    exit: extern "efiapi" fn(),
    unload_image: extern "efiapi" fn(),
    pub exit_bootsrv: extern "efiapi" fn(Handle, usize) -> Status,

    // miscellaneous
    get_next_mono_count: extern "efiapi" fn(),
    stall: extern "efiapi" fn(),
    set_watchdog_timer: extern "efiapi" fn(),

    // driver support
    connect_controller: extern "efiapi" fn(),
    disconnect_controller: extern "efiapi" fn(),

    // open/close protocols
    open_protocol: extern "efiapi" fn(),
    close_protocol: extern "efiapi" fn(),
    open_protocol_info: extern "efiapi" fn(),

    // library
    protocols_per_handle: extern "efiapi" fn(),
    locate_handle_buf: extern "efiapi" fn(),
    pub locate_protocol: extern "efiapi" fn(&Guid, usize, &mut usize) -> Status,
    install_multiple_protocols: extern "efiapi" fn(),
    uninstall_multiple_protocols: extern "efiapi" fn(),

    // crc32
    calc_crc32: extern "efiapi" fn(),

    // miscellaneous
    copy_mem: extern "efiapi" fn(),
    set_mem: extern "efiapi" fn(),
    create_event_ex: extern "efiapi" fn(),
}
