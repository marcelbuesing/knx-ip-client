//! Datapoint Types “2-Octet Signed Value”

use crate::dpt::DptRaw;
use core::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptValue2Count([u8; 2]);

impl From<i16> for DptValue2Count {
    fn from(v: i16) -> Self {
        DptValue2Count(v.to_be_bytes())
    }
}

impl DptRaw for DptValue2Count {
    type ByteArray = [u8; 2];

    fn id() -> (u8, u16) {
        (8, 1)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        self.0
    }
}

impl Display for DptValue2Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i16::from_be_bytes(self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptPercentV16([u8; 2]);

impl DptPercentV16 {
    pub const MIN: f32 = -327.68;
    pub const MAX: f32 = 327.67;
}

impl TryFrom<f32> for DptValue2Count {
    fn try_from(value: f32) -> Self {
        if value < DptPercentV16::MIN || value > DptPercentV16::MAX {
            Err(crate::KnxCoreError::DptValueOutOfRange)
        }

        // resolution (327,67 - -327,67) / 2^16 ~= 0.01
        let scaled = (value / ((DptPercentV16::MAX - DptPercentV16::MIN) / 2 ^ 16)).round() as u16;
        Ok(DptValue2Count(scaled.to_be_bytes))
    }
}

// impl DptRaw for DptPercentV16 {
//     type ByteArray = [u8; 2];

//     fn id() -> (u8, u16) {
//         (8, 2)
//     }

//     fn to_be_bytes(&self) -> Self::ByteArray {
//         self.0
//     }
// }

// TODO remaining Delta Time etc.
