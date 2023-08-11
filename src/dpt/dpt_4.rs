//! Datapoint Types Character Set

use crate::dpt::DptRaw;

/// 3.4 Datapoint Types Character Set
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptCharAscii(u8);

impl DptRaw for DptCharAscii {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (4, 1)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        [self.0 & 0b0111_1111u8; 1]
    }
}

/// 3.4 Datapoint Types Character Set
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptChar8859_1(u8);

impl DptRaw for DptChar8859_1 {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (4, 2)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        [self.0; 1]
    }
}
