use crate::enums::button::knob_ctrl::KnobCtrlKey;
use crate::enums::button::pads::PadKey;
use crate::enums::button::soft_keys::SoftKey;
use crate::errors::mapping::HardwareMappingError;

#[derive(Debug, Copy, Clone)]
pub enum InputGroup {
    Pads(PadKey),
    SoftKeys(SoftKey),
    KnobCtrl(KnobCtrlKey),
    Knob(u8),
    Up,
    Down,
    Right,
    Left,
    StopAllClips,
    Shift,
    ResumePause,
    Start,
}

impl From<InputGroup> for u8 {
    fn from(value: InputGroup) -> Self {
        match value {
            InputGroup::Pads(x) => x.into(),
            InputGroup::SoftKeys(x) => x.into(),
            InputGroup::KnobCtrl(x) => x.into(),
            InputGroup::Up => 64,
            InputGroup::Down => 65,
            InputGroup::Right => 67,
            InputGroup::Left => 66,
            InputGroup::StopAllClips => 81,
            InputGroup::Shift => 98,
            InputGroup::ResumePause => 91,
            InputGroup::Start => 92,
            InputGroup::Knob(index) => index + 47,
        }
    }
}

impl TryFrom<u8> for InputGroup {
    type Error = HardwareMappingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            64 => Ok(Self::Up),
            65 => Ok(Self::Down),
            66 => Ok(Self::Left),
            67 => Ok(Self::Right),
            81 => Ok(Self::StopAllClips),
            98 => Ok(Self::Shift),
            91 => Ok(Self::ResumePause),
            93 => Ok(Self::Start),
            0..=39 => PadKey::try_from(value)
                .map_or(Err(Self::Error::InvalidPadIndex(value)), |k| {
                    Ok(Self::Pads(k))
                }),
            48..=55 => Ok(Self::Knob(value - 47)),
            _ => SoftKey::try_from(value).map_or_else(
                |_| {
                    KnobCtrlKey::try_from(value)
                        .map_or(Err(Self::Error::InvalidKnobCtrlIndex(value)), |k| {
                            Ok(Self::KnobCtrl(k))
                        })
                },
                |k| Ok(Self::SoftKeys(k)),
            ),
        }
    }
}
