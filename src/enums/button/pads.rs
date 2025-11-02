use crate::errors::mapping::HardwareMappingError;
use bon::Builder;

#[derive(Debug, Copy, Clone)]
pub enum PadKey {
    Grid(u8, u8),
}

#[derive(Builder)]
pub struct PadData {
    n_of_rows: Option<u8>,
    n_of_columns: Option<u8>,
    index: u8,
}

impl PadKey {
    #[must_use]
    pub const fn get_row(self) -> u8 {
        match self {
            Self::Grid(row, _) => row,
        }
    }

    #[must_use]
    pub const fn get_col(self) -> u8 {
        match self {
            Self::Grid(row, _) => row,
        }
    }
    #[must_use]
    pub fn get_index(self) -> u8 {
        self.into()
    }
}

impl From<&PadKey> for u8 {
    fn from(key: &PadKey) -> Self {
        match key {
            PadKey::Grid(row, col) => {
                const ROWS: u8 = 5;
                const COLS: u8 = 8;

                (ROWS - row) * COLS + (col - 1)
            }
        }
    }
}

impl From<PadKey> for u8 {
    fn from(key: PadKey) -> Self {
        (&key).into()
    }
}

impl TryFrom<PadData> for PadKey {
    type Error = HardwareMappingError;
    fn try_from(data: PadData) -> Result<Self, Self::Error> {
        let rows = data.n_of_rows.unwrap_or(5);
        let cols = data.n_of_columns.unwrap_or(8);

        if data.index < rows * cols {
            let row = rows - (data.index / cols);
            let col = (data.index % cols) + 1;
            Ok(Self::Grid(row, col))
        } else {
            Err(Self::Error::InvalidPadIndex(data.index))
        }
    }
}

impl TryFrom<u8> for PadKey {
    type Error = HardwareMappingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        PadData::builder()
            .index(value)
            .n_of_rows(5)
            .n_of_columns(8)
            .build()
            .try_into()
    }
}
