use midi_msg::ParseError;
use midir::{ConnectError, InitError, MidiInput, MidiOutput, SendError};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ChannelCreationError {
    #[error("Port {0} not in valid range, check the available range of ports")]
    PortOutOfRange(usize),
    #[error("error {source} when initializing channel")]
    InitializingChannel {
        #[source]
        source: InitError,
    },
    #[error("error {source} when trying to establish a connection")]
    EstablishingInputConnection {
        #[source]
        source: ConnectError<MidiInput>,
    },
    #[error("error {source} when trying to establish a connection")]
    EstablishingOutputConnection {
        #[source]
        source: ConnectError<MidiOutput>,
    },
    #[error("error {source} when trying to initialize led")]
    LedInitialization {
        #[source]
        source: TransmissionError,
    },
}

#[derive(Error, Debug)]
pub enum TransmissionError {
    #[error("error {source} when transmitting {:?}", data)]
    Send {
        data: Vec<u8>,
        #[source]
        source: SendError,
    },
    #[error("error {source} when decoding {:?}", data)]
    Receive {
        data: Vec<u8>,
        #[source]
        source: ParseError,
    },
}
