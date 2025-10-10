use crate::{console::key::InputKey, guid::Guid, services::event::Event, status::Status};

#[repr(C)]
pub struct SimpleTextInput {
    pub reset: extern "efiapi" fn(&Self, bool) -> Status,
    pub read_key_stroke: extern "efiapi" fn(&Self, &mut InputKey) -> Status,
    pub wait_for_key: Event,
}

impl SimpleTextInput {
    pub const GUID: Guid = Guid::new(
        0x387477c1,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
