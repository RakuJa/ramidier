use num_enum::TryFromPrimitive;

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum KnobCtrlKey {
    Volume = 68,
    Pan = 69,
    Send = 70,
    Device = 71,
}

impl From<KnobCtrlKey> for u8 {
    fn from(k: KnobCtrlKey) -> Self {
        k as Self
    }
}
