#[repr(transparent)]
pub struct Event(pub usize);

#[repr(transparent)]
pub struct EventType(u32);

impl EventType {
    pub const TIMER: Self = Self(0x8000_0000);
    pub const RUNTIME: Self = Self(0x4000_0000);
    pub const NOTIFY_WAIT: Self = Self(0x0000_0100);
    pub const NOTIFY_SIGNAL: Self = Self(0x0000_0200);
    pub const EXIT_BOOT_SRV: Self = Self(0x0000_0201);
    pub const VIRTUAL_ADDR_CHANGE: Self = Self(0x6000_0202);
}

pub type EventNotify = extern "efiapi" fn(Event, usize);
