use crate::memory;

#[repr(C)]
pub struct Descriptor {
    pub r#type: u32,
    pub physical_start: memory::addr::Physical,
    pub virtual_start: memory::addr::Virtual,
    pub num_pages: u64,
    pub attr: u64,
}

#[repr(transparent)]
pub struct Attribute(u64);

impl Attribute {
    pub const UC: Self = Self(0x1);
    pub const WC: Self = Self(0x2);
    pub const WT: Self = Self(0x4);
    pub const WB: Self = Self(0x8);
    pub const UCE: Self = Self(0x10);
    pub const WP: Self = Self(0x1000);
    pub const RP: Self = Self(0x2000);
    pub const XP: Self = Self(0x4000);
    pub const NV: Self = Self(0x8000);
    pub const MORE_RELIABLE: Self = Self(0x10_000);
    pub const RO: Self = Self(0x20_000);
    pub const SP: Self = Self(0x40_000);
    pub const CPU_CRYPTO: Self = Self(0x80_000);
    pub const HOT_PLUG: Self = Self(0x100_000);
    pub const RUNTIME: Self = Self(0x8000_0000_0000_0000);
    pub const ISA_VALID: Self = Self(0x4000_0000_0000_0000);
    pub const ISA_MASK: Self = Self(0x0fff_f000_0000_0000);
}
