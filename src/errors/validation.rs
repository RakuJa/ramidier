use thiserror::Error;
#[derive(Error, Debug)]
pub enum LedValidationError {
    #[error("Number {0} not in valid range, check the available range of numbers")]
    InvalidNumber(u8),
}
