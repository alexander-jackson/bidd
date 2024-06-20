use color_eyre::eyre::Result;

mod bluetooth;

use crate::bluetooth::BluetoothController;

const HEADPHONE_NAME: &'static str = "WH-1000XM4";

fn setup() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().with_ansi(true).init();

    Ok(())
}

fn main() -> Result<()> {
    setup()?;

    let controller = BluetoothController::default();
    let devices = controller.get_connected_devices()?;

    if !devices.iter().any(|device| device.contains(HEADPHONE_NAME)) {
        controller.disable_bluetooth()?;

        tracing::info!("Headphones are not on, disabled Bluetooth");
    }

    Ok(())
}
