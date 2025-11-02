use crate::enums::button::input_group::InputGroup;
use midi_msg::Channel;

#[derive(Debug)]
pub struct MidiInputData {
    pub channel: Channel,
    pub input_group: InputGroup,
    pub value: u8,
}
