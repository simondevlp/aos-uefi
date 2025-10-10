#[repr(transparent)]
pub struct Tpl(pub usize);

impl Tpl {
    pub const APPLICATION: Tpl = Tpl(4);
    pub const CALLBACK: Tpl = Tpl(8);
    pub const NOTIFY: Tpl = Tpl(16);
    pub const HIGH_LEVEL: Tpl = Tpl(31);
}
