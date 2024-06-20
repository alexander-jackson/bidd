use std::ffi::OsStr;
use std::process::Command;

use color_eyre::eyre::{eyre, Result};

const HEADPHONE_NAME: &'static str = "WH-1000XM4";

#[derive(Default)]
struct BluetoothController;

impl BluetoothController {
    /// Runs the underlying command with the specified arguments.
    fn run_command<I, S>(&self, args: I) -> Result<Vec<u8>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let command = Command::new("blueutil").args(args).spawn()?;
        let output = command.wait_with_output()?;

        if !output.status.success() {
            return Err(eyre!("Failed to run `blueutil` command"));
        }

        Ok(output.stdout)
    }

    fn get_connected_devices(&self) -> Result<Vec<String>> {
        let output = self.run_command(["--connected"])?;
        let output = std::str::from_utf8(&output)?;

        let lines = output.lines().map(|line| line.to_owned()).collect();

        Ok(lines)
    }

    fn disable_bluetooth(&self) -> Result<()> {
        self.run_command(["--power", "0"])?;

        Ok(())
    }
}

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
