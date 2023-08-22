//! Datapoint Types “8-Bit Unsigned Value”

use snafu::{whatever, Whatever};

use crate::dpt::DptRaw;
use core::fmt::{self, Display};

/// DPT_Scaling
///
/// ID: 5.001\
/// Unit: %\
/// Range: 0..100\
/// Resolution: 0.4%\
/// 3.5.1 Scaled values
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DptScaling(pub u8);

impl TryFrom<f32> for DptScaling {
    type Error = Whatever;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0.0..=100.0).contains(&value) {
            whatever!("Value out of range")
        } else {
            // resolution 100 / 255 ~= 0.4
            let scaled = (value / (100.0 / 255.0)).round() as u8;
            Ok(DptScaling(scaled))
        }
    }
}

impl DptRaw for DptScaling {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (5, 1)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        [self.0; 1]
    }
}

impl Display for DptScaling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let physical = self.0 as f32 * (100.0 / 255.0);
        write!(f, "{}%", physical)
    }
}

/// DPT_Angle
///
/// ID: 5.003\
/// Unit: °\
/// Range: 0..360\
/// Resolution: 1.4°\
/// 3.5.1 Scaled values
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DptAngle(u8);

impl TryFrom<f32> for DptAngle {
    type Error = Whatever;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0.0..=100.0).contains(&value) {
            whatever!("Value out of range")
        } else {
            // resolution 360 / 255 ~= 1.4
            let scaled = (value / (360.0 / 255.0)).round() as u8;
            Ok(DptAngle(scaled))
        }
    }
}

impl DptRaw for DptAngle {
    type ByteArray = [u8; 1];

    fn to_be_bytes(&self) -> Self::ByteArray {
        [self.0; 1]
    }

    fn id() -> (u8, u16) {
        (5, 3)
    }
}

impl Display for DptAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let physical = self.0 as f32 * (360.0 / 255.0);
        write!(f, "{}°", physical)
    }
}

/// DPT_Percent_U8
///
/// ID: 5.004\
/// Unit: %\
/// Range: 0..255\
/// Resolution: 1%\
/// 3.5.1 Scaled values
///
/// ```rust
/// use knx_core::{DptRaw, DptPercentU8};
/// use std::convert::TryFrom;
/// assert_eq!(DptPercentU8::try_from(50).unwrap().to_be_bytes(), [0x32u8; 1]);
/// assert_eq!(DptPercentU8::try_from(100).unwrap().to_be_bytes(), [0x64u8; 1]);
/// assert_eq!(DptPercentU8::try_from(110), Err(knx_core::KnxCoreError::DptValueOutOfRange));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DptPercentU8(u8);

impl TryFrom<u8> for DptPercentU8 {
    type Error = Whatever;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            whatever!("Value out of range")
        } else {
            Ok(DptPercentU8(value))
        }
    }
}

impl DptRaw for DptPercentU8 {
    type ByteArray = [u8; 1];
    fn to_be_bytes(&self) -> Self::ByteArray {
        [self.0; 1]
    }

    fn id() -> (u8, u16) {
        (5, 4)
    }
}

impl Display for DptPercentU8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

/// DPT_DecimalFactor
///
/// ID: 5.005\
/// Unit: ratio\
/// 3.5.1 Scaled values
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DptDecimalFactor(u8);

impl DptRaw for DptDecimalFactor {
    type ByteArray = [u8; 1];

    fn to_be_bytes(&self) -> Self::ByteArray {
        [self.0; 1]
    }

    fn id() -> (u8, u16) {
        (5, 5)
    }
}
