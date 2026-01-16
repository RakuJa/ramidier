use crate::enums::button::base_input_group::BaseInputGroup;
use midi_msg::Channel;

#[derive(Debug)]
pub struct MidiInputData {
    pub channel: Channel,
    pub input_group: BaseInputGroup,
    pub value: u8,
}
