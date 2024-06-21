use std::time::Duration;

use color_eyre::eyre::Result;

mod bluetooth;
mod blueutil;

use crate::bluetooth::{BluetoothController, BluetoothStatus};
use crate::blueutil::BlueutilController;

fn setup() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().with_ansi(true).init();

    Ok(())
}

#[derive(Default)]
struct LifecycleHandler<C> {
    controller: C,
}

impl<C> LifecycleHandler<C>
where
    C: BluetoothController,
{
    fn run(&self) -> Result<()> {
        let status = self.controller.get_bluetooth_status()?;

        tracing::info!(?status, "Successfully got the Bluetooth details");

        if status != BluetoothStatus::Disabled {
            let devices = self.controller.get_connected_devices()?;

            if devices.is_empty() {
                self.controller.disable_bluetooth()?;

                tracing::info!("No devices are connected, disabled Bluetooth");
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    setup()?;

    let handler: LifecycleHandler<BlueutilController> = LifecycleHandler::default();

    loop {
        handler.run()?;

        // Wait for a while to check again
        std::thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests;
