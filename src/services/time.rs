#[repr(C)]
#[derive(Clone, Copy)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pad1: u8,
    pub nanosecond: u32,
    pub timezone: i16,
    pub daylight: u8,
    pad2: u8,
}

#[repr(C)]
pub enum TimerDelay {
    Cancel,
    Periodic,
    Relative,
}
