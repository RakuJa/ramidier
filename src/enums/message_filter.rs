use midir::Ignore;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An enum that is used to specify what kind of MIDI messages should
/// be ignored when receiving messages.
pub enum MessageFilter {
    None = 0x00,
    Sysex = 0x01,
    Time = 0x02,
    SysexAndTime = 0x03,
    ActiveSense = 0x04,
    SysexAndActiveSense = 0x05,
    TimeAndActiveSense = 0x06,
    All = 0x07,
}

impl From<Ignore> for MessageFilter {
    fn from(ignore: Ignore) -> Self {
        match ignore {
            Ignore::None => Self::None,
            Ignore::Sysex => Self::Sysex,
            Ignore::Time => Self::Time,
            Ignore::SysexAndTime => Self::SysexAndTime,
            Ignore::ActiveSense => Self::ActiveSense,
            Ignore::SysexAndActiveSense => Self::SysexAndActiveSense,
            Ignore::TimeAndActiveSense => Self::TimeAndActiveSense,
            Ignore::All => Self::All,
        }
    }
}

impl From<MessageFilter> for Ignore {
    fn from(filter: MessageFilter) -> Self {
        match filter {
            MessageFilter::None => Self::None,
            MessageFilter::Sysex => Self::Sysex,
            MessageFilter::Time => Self::Time,
            MessageFilter::SysexAndTime => Self::SysexAndTime,
            MessageFilter::ActiveSense => Self::ActiveSense,
            MessageFilter::SysexAndActiveSense => Self::SysexAndActiveSense,
            MessageFilter::TimeAndActiveSense => Self::TimeAndActiveSense,
            MessageFilter::All => Self::All,
        }
    }
}
