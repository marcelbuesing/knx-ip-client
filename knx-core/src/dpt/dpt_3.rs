//! Datapoint Types B1U3

use crate::dpt::DptRaw;
use core::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptControlDimmingC {
    Increase,
    Decrease,
}

/// 3.3.1 DPT_Control_Dimming
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptControlDimming {
    pub c: DptControlDimmingC,
    /// 0b000 = break
    /// 0b001..=0b111 = step
    pub step_code: u8,
}

impl DptRaw for DptControlDimming {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (3, 7)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        let c = match self.c {
            DptControlDimmingC::Increase => 0x00u8,
            DptControlDimmingC::Decrease => 0x01u8,
        };
        [c << 3 | (self.step_code & 0b111u8); 1]
    }
}

impl Display for DptControlDimming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.c {
            DptControlDimmingC::Increase => write!(f, "increase"),
            DptControlDimmingC::Decrease => write!(f, "decrease"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptControlBlindsC {
    Up,
    Down,
}

/// 3.3.2 DPT_Control_Blinds
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptControlBlinds {
    pub c: DptControlBlindsC,
    /// 0b000 = break
    /// 0b001..=0b111 = step
    pub step_code: u8,
}

impl DptRaw for DptControlBlinds {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (3, 8)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        let c = match self.c {
            DptControlBlindsC::Up => 0x00u8,
            DptControlBlindsC::Down => 0x01u8,
        };
        [c << 3 | (self.step_code & 0b111u8); 1]
    }
}

impl Display for DptControlBlinds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.c {
            DptControlBlindsC::Up => write!(f, "up"),
            DptControlBlindsC::Down => write!(f, "down"),
        }
    }
}
