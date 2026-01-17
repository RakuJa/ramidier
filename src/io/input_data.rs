use midi_msg::Channel;

#[derive(Debug)]
pub struct MidiInputData<C> {
    pub channel: Channel,
    pub input_group: C,
    pub value: u8,
}
