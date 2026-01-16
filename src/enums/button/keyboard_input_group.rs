use crate::errors::mapping::HardwareMappingError;

#[derive(Debug, Copy, Clone)]
pub enum KeyboardInputGroup {
    Key(u8),
}

impl From<KeyboardInputGroup> for u8 {
    fn from(value: KeyboardInputGroup) -> Self {
        match value {
            KeyboardInputGroup::Key(x) => x,
        }
    }
}

impl TryFrom<u8> for KeyboardInputGroup {
    type Error = HardwareMappingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            48..=72 => Ok(Self::Key(value - 47)),
            _ => Err(Self::Error::InvalidKeyboardKeyIndex(value)),
        }
    }
}
