use crate::enums::led_light::color::LedColor;
use crate::enums::led_light::mode::LedMode;
use crate::errors::io::{ChannelCreationError, TransmissionError};
use crate::io::channel::Channel;
use bon::bon;
use midir::{MidiOutput, MidiOutputConnection};

pub struct ChannelOutput {
    output_connection: MidiOutputConnection,
}

#[bon]
impl ChannelOutput {
    #[builder]
    pub fn new(
        port: Option<usize>,
        initialize_note_led: Option<bool>,
        port_name: Option<&str>,
    ) -> Result<Self, ChannelCreationError> {
        let midi_output = Self::get_midi()?;
        let out_ports = midi_output.ports();
        let chosen_port = port.unwrap_or(2);
        let out_port = out_ports
            .get(chosen_port)
            .ok_or(ChannelCreationError::PortOutOfRange(chosen_port))?;

        let mut output_connection = midi_output
            .connect(out_port, port_name.unwrap_or("akai-midir-write-output"))
            .map_err(|e| ChannelCreationError::EstablishingOutputConnection { source: e })?;
        if initialize_note_led.unwrap_or(false) {
            initialize_pads_led(&mut output_connection)
                .map_err(|e| ChannelCreationError::LedInitialization { source: e })?;
        }
        Ok(Self { output_connection })
    }

    /// # Errors
    ///
    /// Will return `TransmissionError` if data is not valid or there are low-level issues communicating with the device
    pub fn send(&mut self, data: &[u8]) -> Result<(), TransmissionError> {
        send(&mut self.output_connection, data)
    }

    /// # Errors
    ///
    /// Will return `TransmissionError` if there are low-level issues communicating with the device
    pub fn initialize_notes_led(&mut self) -> Result<(), TransmissionError> {
        initialize_pads_led(&mut self.output_connection)
    }

    /// # Errors
    ///
    /// Will return `TransmissionError` if there are low-level issues communicating with the device
    pub fn set_all_pads_color(
        &mut self,
        led_mode: LedMode,
        color: LedColor,
    ) -> Result<(), TransmissionError> {
        initialize_pads_led(&mut self.output_connection)?;
        for i in 0..=86 {
            self.set_pad_led(led_mode, i, color)?;
        }
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `TransmissionError` if there are low-level issues communicating with the device
    pub fn set_pad_led<T>(
        &mut self,
        led_mode: LedMode,
        note: T,
        color: LedColor,
    ) -> Result<(), TransmissionError>
    where
        T: Into<u8>,
    {
        let led_msg = [led_mode.into(), note.into(), color.into()];
        self.send(led_msg.as_ref())
    }
}

impl Channel for ChannelOutput {
    #[allow(refining_impl_trait)]
    fn get_midi() -> Result<MidiOutput, ChannelCreationError> {
        MidiOutput::new("akai midir writing output")
            .map_err(|e| ChannelCreationError::InitializingChannel { source: e })
    }
}

fn initialize_pads_led(
    output_connection: &mut MidiOutputConnection,
) -> Result<(), TransmissionError> {
    let init_sysex: Vec<u8> = vec![
        0xF0, // SysEx start
        0x47, // Akai manufacturer ID
        0x7F, // Device ID (all devices)
        0x29, // Product ID for APC Key 25 mk2
        0x60, // Message type: Initialize
        0x00, 0x04, // Application version (can be any value)
        0x42, // Content version
        0x08, 0x02, 0x01, // Mode/configuration
        0xF7, // SysEx end
    ];
    send(output_connection, &init_sysex)
}

/// # Errors
///
/// Will return `TransmissionError` if it's not a valid MIDI message or there are low-level issue communicating with the device
fn send(
    output_connection: &mut MidiOutputConnection,
    data: &[u8],
) -> Result<(), TransmissionError> {
    output_connection
        .send(data)
        .map_err(|e| TransmissionError::Send {
            data: data.to_vec(),
            source: e,
        })
}
