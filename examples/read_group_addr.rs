//! This example shows how to read group address values.
//!
//! `cargo run --example read_group_address`
//!
use clap::Parser;
use knx_ip_client::{dp_types::PdtKnxFloat, transport::udp::UdpClient};
use log::debug;
use snafu::{OptionExt, ResultExt, Whatever};
use std::{sync::Arc, time::Duration};
use tokio::{select, time::timeout};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "2/0/0")]
    group_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Whatever> {
    env_logger::init();
    let args = Args::parse();

    let devices = UdpClient::search(Duration::from_millis(100)).await?;
    let device = devices.first().whatever_context("No device found")?;
    println!("Discovered device: {:?}", device.control_endpoint.address);

    let client = Arc::new(UdpClient::connect(device.control_endpoint.address).await?);

    loop {
        select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {
                let group_address = args.group_address.as_str().try_into().unwrap();
                let value = timeout(Duration::from_secs(1), client.read_group_address_value(group_address)).await.whatever_context("Failed to read group address")??;
                debug!("Read value {:?}", value);
                let pdt = PdtKnxFloat::temp_from_bytes(value)?;
                println!("Read value {:?}", pdt.get_value());
            },
            _ = tokio::signal::ctrl_c() => {
                println!("Sending disconnect");
                client.disconnect().await?;
                break;
            }
        }
    }

    Ok(())
}
