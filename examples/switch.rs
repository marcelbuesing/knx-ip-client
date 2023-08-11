use clap::{Parser, ValueEnum};
use knx_ip_client::{
    dpt::{DptRaw, DptSwitch},
    KnxAddress, UdpClient,
};
use snafu::{OptionExt, Whatever};
use std::{sync::Arc, time::Duration};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SwitchState {
    /// Set to OFF position
    Off,
    /// Set to ON position
    On,
}

impl From<SwitchState> for DptSwitch {
    fn from(state: SwitchState) -> Self {
        match state {
            SwitchState::Off => DptSwitch::Off,
            SwitchState::On => DptSwitch::On,
        }
    }
}

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Group Address Switch e.g. 0/1/1
    #[arg(short, long)]
    group_address: String,

    #[arg(value_enum)]
    switch: SwitchState,
}

#[tokio::main]
async fn main() -> Result<(), Whatever> {
    env_logger::init();
    let args = Args::parse();

    let devices = UdpClient::search(Duration::from_millis(100)).await?;
    let device = devices.first().whatever_context("No device found")?;
    println!("Discovered endpoint device: {:?}", device.control_endpoint.address);

    let client = Arc::new(UdpClient::connect(device.control_endpoint.address).await?);

    let group_address = KnxAddress::try_from(args.group_address.as_str()).unwrap();
    let dpt: DptSwitch = args.switch.into();
    println!("Writing value `{:?}` to group address {:?}", args.switch, group_address);
    client.write_group_address_value(group_address, dpt.to_be_bytes().to_vec()).await?;

    Ok(())
}
