use num_enum::TryFromPrimitive;

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum SoftKey {
    ClipStop = 82,
    Solo = 83,
    Mute = 84,
    RecArm = 85,
    Select = 86,
}

impl From<SoftKey> for u8 {
    fn from(key: SoftKey) -> Self {
        key as Self
    }
}
