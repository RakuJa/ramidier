use thiserror::Error;
#[derive(Error, Debug)]
pub enum HardwareMappingError {
    #[error("Number {0} not in valid range, check the available range of numbers")]
    InvalidPadIndex(u8),
    #[error("Number {0} not in valid range, check the available range of numbers")]
    InvalidKnobCtrlIndex(u8),
    #[error("Number {0} not in valid range, check the available range of numbers")]
    InvalidSoftKeyIndex(u8),
    #[error("Number {0} not in valid range, check the available range of numbers")]
    InvalidKeyboardKeyIndex(u8),
}
