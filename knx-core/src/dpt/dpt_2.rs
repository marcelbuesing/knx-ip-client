//! Datapoint Types B2

use crate::dpt::DptRaw;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptControl {
    NoControl,
    NoControl2,
    ControlFunctionValue0,
    ControlFunctionValue1,
}

impl DptRaw for DptControl {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (2, 1)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptControl::NoControl => [0x00],
            DptControl::NoControl2 => [0x01],
            DptControl::ControlFunctionValue0 => [0x02],
            DptControl::ControlFunctionValue1 => [0x03],
        }
    }
}

pub type DptSwitchControl = DptControl;
pub type DptBoolControl = DptControl;
pub type DptEnableControl = DptControl;
pub type DptRampControl = DptControl;
pub type DptAlarmControl = DptControl;
pub type DptBinaryValueControl = DptControl;
pub type DptStepControl = DptControl;
pub type DptDirection1Control = DptControl;
pub type DptDirection2Control = DptControl;
pub type DptStartControl = DptControl;
pub type DptInvertControl = DptControl;
