use crate::errors::mapping::HardwareMappingError;

#[derive(Debug, Copy, Clone)]
pub enum SoftKey {
    ClipStop = 82,
    Solo = 83,
    Mute = 84,
    RecArm = 85,
    Select = 86,
}

impl TryFrom<u8> for SoftKey {
    type Error = HardwareMappingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            82 => Ok(Self::ClipStop),
            83 => Ok(Self::Solo),
            84 => Ok(Self::Mute),
            85 => Ok(Self::RecArm),
            86 => Ok(Self::Select),
            _ => Err(Self::Error::InvalidSoftKeyIndex(value)),
        }
    }
}

impl From<SoftKey> for u8 {
    fn from(key: SoftKey) -> Self {
        key as Self
    }
}
