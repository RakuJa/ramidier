use crate::errors::mapping::HardwareMappingError;

#[derive(Debug, Copy, Clone)]
pub enum KnobCtrlKey {
    Volume = 68,
    Pan = 69,
    Send = 70,
    Device = 71,
}

impl TryFrom<u8> for KnobCtrlKey {
    type Error = HardwareMappingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            68 => Ok(Self::Volume),
            69 => Ok(Self::Pan),
            70 => Ok(Self::Send),
            71 => Ok(Self::Device),
            _ => Err(Self::Error::InvalidKnobCtrlIndex(value)),
        }
    }
}

impl From<KnobCtrlKey> for u8 {
    fn from(k: KnobCtrlKey) -> Self {
        k as Self
    }
}
