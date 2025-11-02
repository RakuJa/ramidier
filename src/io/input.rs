use crate::enums::button::input_group::InputGroup;
use crate::enums::message_filter::MessageFilter;
use crate::errors::io::{ChannelCreationError, TransmissionError};
use crate::io::channel::Channel;
use crate::io::input_data::MidiInputData;
use bon::bon;
use log::warn;
use midi_msg::{ChannelVoiceMsg, MidiMsg, ReceiverContext};
use midir::{MidiInput, MidiInputConnection, MidiInputPort};

pub struct InputChannel {
    midi_input: MidiInput,
    input_port: MidiInputPort,
}

#[bon]
impl InputChannel {
    #[builder]
    pub fn new(
        port: Option<usize>,
        msg_to_ignore: Option<MessageFilter>,
    ) -> Result<Self, ChannelCreationError> {
        let mut midi_in = Self::get_midi()?;
        midi_in.ignore(msg_to_ignore.unwrap_or(MessageFilter::None).into());
        let in_ports = midi_in.ports();
        let chosen_port = port.unwrap_or(2);
        Ok(Self {
            midi_input: midi_in,
            input_port: in_ports
                .get(chosen_port)
                .ok_or(ChannelCreationError::PortOutOfRange(chosen_port))?
                .clone(),
        })
    }

    /// Very biased listener method. It will try to decode the MIDI messages to a high level representation
    /// and after that it will call the given closure
    /// ```Rust
    ///let midi_in = ramidier::io::input::InputChannel::builder().build()?;
    ///let _conn_in = midi_in.listen(
    ///    Some("midir-input"),
    ///    move |stamp, received_input, data| listener_logic(&mut midi_out, stamp, &received_input, data),
    ///    MyDataStruct{}, // could also be () it there is no need for data
    /// )?;
    /// ```
    /// # Errors
    ///
    /// Will return `ChannelCreationError` if there are low-level issues communicating with the device
    pub fn listen<F, T: Send>(
        self,
        port_name: Option<&str>,
        mut input_handler_callback: F,
        data: T,
    ) -> Result<MidiInputConnection<T>, ChannelCreationError>
    where
        F: FnMut(u64, MidiInputData, &mut T) + Send + 'static,
    {
        let mut ctx = ReceiverContext::new();
        let wrapper = move |timestamp: u64, midi_bytes: &[u8], user_data: &mut T| {
            if let Ok((msg, _len)) =
                MidiMsg::from_midi_with_context(midi_bytes, &mut ctx).map_err(|e| {
                    TransmissionError::Receive {
                        data: vec![],
                        source: e,
                    }
                })
            {
                if let Some(x) = match msg {
                    MidiMsg::ChannelVoice {
                        channel,
                        msg: ChannelVoiceMsg::NoteOn { note, velocity: _ },
                    } => InputGroup::try_from(note)
                        .map(|input_group| MidiInputData {
                            channel,
                            input_group,
                            value: 1u8,
                        })
                        .ok(),
                    MidiMsg::ChannelVoice {
                        channel,
                        msg: ChannelVoiceMsg::NoteOff { note, velocity: _ },
                    } => InputGroup::try_from(note)
                        .map(|input_group| MidiInputData {
                            channel,
                            input_group,
                            value: 0u8,
                        })
                        .ok(),
                    MidiMsg::ChannelVoice {
                        channel,
                        msg: ChannelVoiceMsg::ControlChange { control },
                    } => InputGroup::try_from(control.control())
                        .map(|input_group| MidiInputData {
                            channel,
                            input_group,
                            value: control.value(),
                        })
                        .ok(),
                    _ => None,
                } {
                    // call the original callback
                    input_handler_callback(timestamp, x, user_data);
                } else {
                    warn!(
                        "MIDI message received but not yet implemented, try using `listen_midi_msg`"
                    );
                }
            } else {
                warn!("Message received but could not be decoded, try using `listen_raw` ");
            }
        };
        self.listen_raw(port_name, wrapper, data)
    }

    /// Listener method that will try to decode the received bytes to the MIDI messages
    /// and after that it will call the given closure
    /// ```Rust
    ///let midi_in = ramidier::io::input::InputChannel::builder().build()?;
    ///let _conn_in = midi_in.listen_midi_msg(
    ///    Some("midi-input"),
    ///    move |stamp, midi_msg, data| listener_logic(&mut midi_out, stamp, &midi_mgs, data),
    ///    MyDataStruct{}, // could also be () it there is no need for data
    /// )?;
    /// ```
    /// # Errors
    ///
    /// Will return `ChannelCreationError` if there are low-level issues communicating with the device
    pub fn listen_midi_msg<F, T: Send>(
        self,
        port_name: Option<&str>,
        mut input_handler_callback: F,
        data: T,
    ) -> Result<MidiInputConnection<T>, ChannelCreationError>
    where
        F: FnMut(u64, MidiMsg, &mut T) + Send + 'static,
    {
        let mut ctx = ReceiverContext::new();
        let wrapper =
            move |timestamp: u64, midi_bytes: &[u8], user_data: &mut T| {
                if let Ok((msg, _len)) = MidiMsg::from_midi_with_context(midi_bytes, &mut ctx)
                    .map_err(|e| TransmissionError::Receive {
                        data: vec![],
                        source: e,
                    })
                {
                    // call the original callback
                    input_handler_callback(timestamp, msg, user_data);
                }
            };
        self.listen_raw(port_name, wrapper, data)
    }

    /// Listener method that will will call the given closure every time it receives a midi message. It does not decode the raw bytes.
    /// ```Rust
    ///let midi_in = ramidier::io::input::InputChannel::builder().build()?;
    ///let _conn_in = midi_in.listen_raw(
    ///    Some("midi-input"),
    ///    move |stamp, midi_bytes, data| listener_logic(&mut midi_out, stamp, &midi_bytes, data),
    ///    MyDataStruct{}, // could also be () it there is no need for data
    /// )?;
    /// ```
    /// # Errors
    ///
    /// Will return `ChannelCreationError` if there are low-level issues communicating with the device
    pub fn listen_raw<F, T: Send>(
        self,
        port_name: Option<&str>,
        input_handler_callback: F,
        data: T,
    ) -> Result<MidiInputConnection<T>, ChannelCreationError>
    where
        F: FnMut(u64, &[u8], &mut T) + Send + 'static,
    {
        self.midi_input
            .connect(
                &self.input_port,
                port_name.unwrap_or("akai-midir-read-input"),
                input_handler_callback,
                data,
            )
            .map_err(|e| ChannelCreationError::EstablishingInputConnection { source: e })
    }
}

impl Channel for InputChannel {
    #[allow(refining_impl_trait)]
    fn get_midi() -> Result<MidiInput, ChannelCreationError> {
        MidiInput::new("akai midir reading input")
            .map_err(|e| ChannelCreationError::InitializingChannel { source: e })
    }
}
