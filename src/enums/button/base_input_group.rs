use crate::enums::button::knob_ctrl::KnobCtrlKey;
use crate::enums::button::pads::PadKey;
use crate::enums::button::soft_keys::SoftKey;
use crate::errors::mapping::HardwareMappingError;

#[derive(Debug, Copy, Clone)]
pub enum BaseInputGroup {
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

impl From<BaseInputGroup> for u8 {
    fn from(value: BaseInputGroup) -> Self {
        match value {
            BaseInputGroup::Pads(x) => x.into(),
            BaseInputGroup::SoftKeys(x) => x.into(),
            BaseInputGroup::KnobCtrl(x) => x.into(),
            BaseInputGroup::Up => 64,
            BaseInputGroup::Down => 65,
            BaseInputGroup::Right => 67,
            BaseInputGroup::Left => 66,
            BaseInputGroup::StopAllClips => 81,
            BaseInputGroup::Shift => 98,
            BaseInputGroup::ResumePause => 91,
            BaseInputGroup::Start => 92,
            BaseInputGroup::Knob(index) => index + 47,
        }
    }
}

impl TryFrom<u8> for BaseInputGroup {
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
