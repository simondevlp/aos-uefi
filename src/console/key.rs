#[repr(C)]
pub struct KeyData {
    pub key: InputKey,
    pub state: KeyState,
}

#[repr(C)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: u16,
}

#[repr(C)]
pub struct KeyState {
    pub shift: KeyShiftState,
    pub toggle: KeyToggleState,
}

#[repr(transparent)]
pub struct KeyShiftState(pub u32);

impl KeyShiftState {
    pub const VALID: Self = Self(0x8000_0000);
    pub const RIGHT_SHIFT: Self = Self(0x1);
    pub const LEFT_SHIFT: Self = Self(0x2);
    pub const RIGHT_CTRL: Self = Self(0x4);
    pub const LEFT_CTRL: Self = Self(0x8);
    pub const RIGHT_ALT: Self = Self(0x10);
    pub const LEFT_ALT: Self = Self(0x20);
    pub const RIGHT_LOGO: Self = Self(0x40);
    pub const LEFT_LOGO: Self = Self(0x80);
    pub const MENU: Self = Self(0x100);
    pub const SYS_REQ: Self = Self(0x200);
}

#[repr(transparent)]
pub struct KeyToggleState(pub u8);

impl KeyToggleState {
    pub const VALID: Self = Self(0x80);
    pub const EXPOSED: Self = Self(0x40);
    pub const SCROLL: Self = Self(0x01);
    pub const NUM_LOCK: Self = Self(0x02);
    pub const CAPS_LOCK: Self = Self(0x04);
}
