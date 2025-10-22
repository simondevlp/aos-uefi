#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Physical(pub u64);

#[repr(transparent)]
pub struct Virtual(pub u64);
