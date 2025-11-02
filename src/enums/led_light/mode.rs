use num_enum::TryFromPrimitive;

#[derive(Debug, Copy, Clone, Default, TryFromPrimitive)]
#[repr(u8)]
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
