//! 0.3.07.02 Datapoint Types - 3.1 Datapoint Types B1

use crate::dpt::DptRaw;
use std::fmt::Display;

pub enum DptSwitch {
    Off,
    On,
}

impl DptRaw for DptSwitch {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 1)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptSwitch::Off => [0x00],
            DptSwitch::On => [0x01],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptBool(bool);

impl DptRaw for DptBool {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 2)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        [self.0 as u8; 1]
    }
}

impl Display for DptBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => write!(f, "true"),
            false => write!(f, "false"),
        }
    }
}

impl From<bool> for DptBool {
    fn from(value: bool) -> Self {
        DptBool(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptEnable {
    Disable,
    Enable,
}

impl DptRaw for DptEnable {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 3)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptEnable::Disable => [0x00],
            DptEnable::Enable => [0x01],
        }
    }
}

impl Display for DptEnable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptEnable::Disable => write!(f, "disable"),
            DptEnable::Enable => write!(f, "enable"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptRamp {
    NoRamp,
    Ramp,
}

impl DptRaw for DptRamp {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 4)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptRamp::NoRamp => [0x00],
            DptRamp::Ramp => [0x01],
        }
    }
}

impl Display for DptRamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptRamp::NoRamp => write!(f, "no ramp"),
            DptRamp::Ramp => write!(f, "ramp"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptAlarm {
    NoAlarm,
    Alarm,
}

impl DptRaw for DptAlarm {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 5)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptAlarm::NoAlarm => [0x00],
            DptAlarm::Alarm => [0x01],
        }
    }
}

impl Display for DptAlarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptAlarm::NoAlarm => write!(f, "no alarm"),
            DptAlarm::Alarm => write!(f, "alarm"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptBinaryValue {
    Low,
    High,
}

impl DptRaw for DptBinaryValue {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 6)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptBinaryValue::Low => [0x00],
            DptBinaryValue::High => [0x01],
        }
    }
}

impl Display for DptBinaryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptBinaryValue::Low => write!(f, "low"),
            DptBinaryValue::High => write!(f, "high"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptStep {
    Decrease,
    Increase,
}

impl DptRaw for DptStep {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 7)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptStep::Decrease => [0x00],
            DptStep::Increase => [0x01],
        }
    }
}

impl Display for DptStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptStep::Decrease => write!(f, "decrease"),
            DptStep::Increase => write!(f, "increase"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptUpDown {
    Up,
    Down,
}

impl DptRaw for DptUpDown {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 8)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptUpDown::Up => [0x00],
            DptUpDown::Down => [0x01],
        }
    }
}

impl Display for DptUpDown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptUpDown::Up => write!(f, "up"),
            DptUpDown::Down => write!(f, "down"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptOpenClose {
    Open,
    Close,
}

impl DptRaw for DptOpenClose {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 9)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptOpenClose::Open => [0x00],
            DptOpenClose::Close => [0x01],
        }
    }
}

impl Display for DptOpenClose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptOpenClose::Open => write!(f, "open"),
            DptOpenClose::Close => write!(f, "close"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptStart {
    Stop,
    Start,
}

impl DptRaw for DptStart {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 10)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptStart::Stop => [0x00],
            DptStart::Start => [0x01],
        }
    }
}

impl Display for DptStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptStart::Stop => write!(f, "stop"),
            DptStart::Start => write!(f, "start"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptState {
    Inactive,
    Active,
}

impl DptRaw for DptState {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 11)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptState::Inactive => [0x00],
            DptState::Active => [0x01],
        }
    }
}

impl Display for DptState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptState::Inactive => write!(f, "inactive"),
            DptState::Active => write!(f, "active"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptInvert {
    NotInverted,
    Inverted,
}

impl DptRaw for DptInvert {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 12)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptInvert::NotInverted => [0x00],
            DptInvert::Inverted => [0x01],
        }
    }
}

impl Display for DptInvert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptInvert::NotInverted => write!(f, "not inverted"),
            DptInvert::Inverted => write!(f, "inverted"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptDimSendStyle {
    StartStop,
    Cyclically,
}

impl DptRaw for DptDimSendStyle {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 13)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptDimSendStyle::StartStop => [0x00],
            DptDimSendStyle::Cyclically => [0x01],
        }
    }
}

impl Display for DptDimSendStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptDimSendStyle::StartStop => write!(f, "start stop"),
            DptDimSendStyle::Cyclically => write!(f, "cyclically"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptInputSource {
    Fixed,
    Calculated,
}

impl DptRaw for DptInputSource {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 14)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptInputSource::Fixed => [0x00],
            DptInputSource::Calculated => [0x01],
        }
    }
}

impl Display for DptInputSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptInputSource::Fixed => write!(f, "fixed"),
            DptInputSource::Calculated => write!(f, "calculated"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptReset {
    /// Dummy
    NoAction,
    /// Trigger
    Reset,
}

impl DptRaw for DptReset {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 15)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptReset::NoAction => [0x00],
            DptReset::Reset => [0x01],
        }
    }
}

impl Display for DptReset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptReset::NoAction => write!(f, "no action"),
            DptReset::Reset => write!(f, "reset"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptAck {
    /// Dummy
    NoAction,
    /// Trigger
    Ack,
}

impl DptRaw for DptAck {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 16)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptAck::NoAction => [0x00],
            DptAck::Ack => [0x01],
        }
    }
}

impl Display for DptAck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptAck::NoAction => write!(f, "no action"),
            DptAck::Ack => write!(f, "ack"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptTrigger {
    /// Dummy
    NoAction,
    /// Trigger
    Trigger,
}

impl DptRaw for DptTrigger {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 17)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptTrigger::NoAction => [0x00],
            DptTrigger::Trigger => [0x01],
        }
    }
}

impl Display for DptTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptTrigger::NoAction => write!(f, "no action"),
            DptTrigger::Trigger => write!(f, "trigger"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptOccupancy {
    NotOccupied,
    Occupancy,
}

impl DptRaw for DptOccupancy {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 18)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptOccupancy::NotOccupied => [0x00],
            DptOccupancy::Occupancy => [0x01],
        }
    }
}

impl Display for DptOccupancy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptOccupancy::NotOccupied => write!(f, "not occupied"),
            DptOccupancy::Occupancy => write!(f, "occupancy"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptWindowDoor {
    Closed,
    Open,
}

impl DptRaw for DptWindowDoor {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 19)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptWindowDoor::Closed => [0x00],
            DptWindowDoor::Open => [0x01],
        }
    }
}

impl Display for DptWindowDoor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptWindowDoor::Closed => write!(f, "closed"),
            DptWindowDoor::Open => write!(f, "open"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptLogicalFunction {
    Or,
    And,
}

impl DptRaw for DptLogicalFunction {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 21)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptLogicalFunction::Or => [0x00],
            DptLogicalFunction::And => [0x01],
        }
    }
}

impl Display for DptLogicalFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptLogicalFunction::Or => write!(f, "or"),
            DptLogicalFunction::And => write!(f, "and"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptSceneAb {
    SceneA,
    SceneB,
}

impl DptRaw for DptSceneAb {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 22)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptSceneAb::SceneA => [0x00],
            DptSceneAb::SceneB => [0x01],
        }
    }
}

impl Display for DptSceneAb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptSceneAb::SceneA => write!(f, "scene A"),
            DptSceneAb::SceneB => write!(f, "scene B"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptShutterBlindsMode {
    /// Shutter
    OnlyMoveUpDownMode,
    /// Blind
    MoveUpDownAndStepMode,
}

impl DptRaw for DptShutterBlindsMode {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 23)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptShutterBlindsMode::OnlyMoveUpDownMode => [0x00],
            DptShutterBlindsMode::MoveUpDownAndStepMode => [0x01],
        }
    }
}

impl Display for DptShutterBlindsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptShutterBlindsMode::OnlyMoveUpDownMode => write!(f, "only move up/down mode"),
            DptShutterBlindsMode::MoveUpDownAndStepMode => write!(f, "move up/down and step mode"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DptHeatCool {
    Heat,
    Cool,
}

impl DptRaw for DptHeatCool {
    type ByteArray = [u8; 1];

    fn id() -> (u8, u16) {
        (1, 100)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        match self {
            DptHeatCool::Heat => [0x00],
            DptHeatCool::Cool => [0x01],
        }
    }
}

impl Display for DptHeatCool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DptHeatCool::Heat => write!(f, "heat"),
            DptHeatCool::Cool => write!(f, "cool"),
        }
    }
}
