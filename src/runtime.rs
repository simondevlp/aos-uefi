use crate::table::TableHeader;

#[repr(C)]
pub struct RuntimeServices {
    hdr: TableHeader,

    // time
    get_time: usize,
    set_time: usize,
    get_wakeup_time: usize,
    set_wakeup_time: usize,

    // virtual mem
    set_virt_addr_map: usize,
    convert_ptr: usize,

    // variable
    get_var: usize,
    get_next_var_name: usize,
    set_var: usize,

    // miscellaneous
    get_next_high_mono_count: usize,
    reset: usize,

    // capsule
    update_capsule: usize,
    query_capsule_capabilities: usize,

    // miscellaneous
    query_var_info: usize,
}
