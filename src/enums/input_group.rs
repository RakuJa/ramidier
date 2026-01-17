use crate::enums::button::knob_ctrl::KnobCtrlKey;
use crate::enums::button::pads::PadKey;
use crate::enums::button::soft_keys::SoftKey;
use crate::errors::mapping::HardwareMappingError;
use crate::io::input_data::MidiInputData;
use midi_msg::{ChannelVoiceMsg, MidiMsg};

pub struct PadsAndKnobsChannel;
pub struct KeyboardChannel;

pub trait ChannelKind {
    type Group;

    fn decode(msg: &MidiMsg) -> Option<MidiInputData<Self::Group>>;
}

impl ChannelKind for PadsAndKnobsChannel {
    type Group = PadsAndKnobsInputGroup;

    fn decode(msg: &MidiMsg) -> Option<MidiInputData<Self::Group>> {
        match msg {
            MidiMsg::ChannelVoice {
                channel,
                msg: ChannelVoiceMsg::NoteOn { note, velocity: _ },
            } => PadsAndKnobsInputGroup::try_from(*note)
                .map(|input_group| MidiInputData {
                    channel: *channel,
                    input_group,
                    value: 1u8,
                })
                .ok(),
            MidiMsg::ChannelVoice {
                channel,
                msg: ChannelVoiceMsg::NoteOff { note, velocity: _ },
            } => PadsAndKnobsInputGroup::try_from(*note)
                .map(|input_group| MidiInputData {
                    channel: *channel,
                    input_group,
                    value: 0u8,
                })
                .ok(),
            MidiMsg::ChannelVoice {
                channel,
                msg: ChannelVoiceMsg::ControlChange { control },
            } => PadsAndKnobsInputGroup::try_from(control.control())
                .map(|input_group| MidiInputData {
                    channel: *channel,
                    input_group,
                    value: control.value(),
                })
                .ok(),
            _ => None,
        }
    }
}

impl ChannelKind for KeyboardChannel {
    type Group = KeyboardInputGroup;

    fn decode(msg: &MidiMsg) -> Option<MidiInputData<Self::Group>> {
        match msg {
            MidiMsg::ChannelVoice {
                channel,
                msg: ChannelVoiceMsg::NoteOn { note, velocity: _ },
            } => KeyboardInputGroup::try_from(*note)
                .map(|input_group| MidiInputData {
                    channel: *channel,
                    input_group,
                    value: 1u8,
                })
                .ok(),
            MidiMsg::ChannelVoice {
                channel,
                msg: ChannelVoiceMsg::NoteOff { note, velocity: _ },
            } => KeyboardInputGroup::try_from(*note)
                .map(|input_group| MidiInputData {
                    channel: *channel,
                    input_group,
                    value: 0u8,
                })
                .ok(),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PadsAndKnobsInputGroup {
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

impl From<PadsAndKnobsInputGroup> for u8 {
    fn from(value: PadsAndKnobsInputGroup) -> Self {
        match value {
            PadsAndKnobsInputGroup::Pads(x) => x.into(),
            PadsAndKnobsInputGroup::SoftKeys(x) => x.into(),
            PadsAndKnobsInputGroup::KnobCtrl(x) => x.into(),
            PadsAndKnobsInputGroup::Up => 64,
            PadsAndKnobsInputGroup::Down => 65,
            PadsAndKnobsInputGroup::Right => 67,
            PadsAndKnobsInputGroup::Left => 66,
            PadsAndKnobsInputGroup::StopAllClips => 81,
            PadsAndKnobsInputGroup::Shift => 98,
            PadsAndKnobsInputGroup::ResumePause => 91,
            PadsAndKnobsInputGroup::Start => 92,
            PadsAndKnobsInputGroup::Knob(index) => index + 47,
        }
    }
}

impl TryFrom<u8> for PadsAndKnobsInputGroup {
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
