use crate::errors::validation::LedValidationError;

#[derive(Debug, Copy, Clone, Default)]
pub enum LedMode {
    On10Percent = 0x90,
    On25Percent = 0x91,
    On50Percent = 0x92,
    On65Percent = 0x93,
    On75Percent = 0x94,
    On90Percent = 0x95,
    #[default]
    On100Percent = 0x96,
    Pulsing1over16 = 0x97,
    Pulsing1over8 = 0x98,
    Pulsing1over4 = 0x99,
    Pulsing1over2 = 0x9A,
    Blinking1over24 = 0x9B,
    Blinking1over16 = 0x9C,
    Blinking1over8 = 0x9D,
    Blinking1over4 = 0x9E,
    Blinking1over2 = 0x9F,
}

impl From<LedMode> for u8 {
    fn from(mode: LedMode) -> Self {
        mode as Self
    }
}

impl TryFrom<u8> for LedMode {
    type Error = LedValidationError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x90u8 => Ok(Self::On10Percent),
            0x91u8 => Ok(Self::On25Percent),
            0x92u8 => Ok(Self::On50Percent),
            0x93u8 => Ok(Self::On65Percent),
            0x94u8 => Ok(Self::On75Percent),
            0x95u8 => Ok(Self::On90Percent),
            0x96u8 => Ok(Self::On100Percent),
            0x97u8 => Ok(Self::Pulsing1over16),
            0x98u8 => Ok(Self::Pulsing1over8),
            0x99u8 => Ok(Self::Pulsing1over4),
            0x9Au8 => Ok(Self::Pulsing1over2),
            0x9Bu8 => Ok(Self::Blinking1over24),
            0x9Cu8 => Ok(Self::Blinking1over16),
            0x9Du8 => Ok(Self::Blinking1over8),
            0x9Eu8 => Ok(Self::Blinking1over4),
            0x9Fu8 => Ok(Self::Blinking1over2),
            _ => Err(LedValidationError::InvalidNumber(val)),
        }
    }
}
