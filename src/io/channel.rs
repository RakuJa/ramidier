use crate::errors::io::ChannelCreationError;
use midir::MidiIO;
use std::collections::HashMap;

pub trait Channel {
    #[allow(dead_code)]
    fn get_available_ports_data() -> Result<HashMap<usize, String>, ChannelCreationError> {
        let midi_in = Self::get_midi()?;
        let in_ports = midi_in.ports();
        Ok(in_ports
            .iter()
            .enumerate()
            .filter_map(|(i, p)| midi_in.port_name(p).map(|p_name| (i, p_name)).ok())
            .collect())
    }

    #[allow(dead_code)]
    fn get_available_ports_indexes() -> Result<Vec<usize>, ChannelCreationError> {
        let midi_in = Self::get_midi()?;
        Ok((0..midi_in.port_count()).collect())
    }
    fn get_midi() -> Result<impl MidiIO, ChannelCreationError>;
}
