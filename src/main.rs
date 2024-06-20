use std::time::Duration;

use color_eyre::eyre::Result;

mod bluetooth;

use crate::bluetooth::{BluetoothController, BluetoothStatus};

const HEADPHONE_NAME: &'static str = "WH-1000XM4";

fn setup() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().with_ansi(true).init();

    Ok(())
}

fn main() -> Result<()> {
    setup()?;

    let controller = BluetoothController::default();

    loop {
        let status = controller.get_bluetooth_status()?;

        tracing::debug!(?status, "Successfully got the Bluetooth details");

        if status != BluetoothStatus::Disabled {
            let devices = controller.get_connected_devices()?;

            if !devices.iter().any(|device| device.contains(HEADPHONE_NAME)) {
                controller.disable_bluetooth()?;

                tracing::info!("Headphones are not on, disabled Bluetooth");
            }
        }

        // Wait for a while to check again
        std::thread::sleep(Duration::from_secs(60));
    }
}
