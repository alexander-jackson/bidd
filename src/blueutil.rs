use std::ffi::OsStr;
use std::process::Command;

use color_eyre::eyre::{eyre, Result};

use crate::bluetooth::{BluetoothController, BluetoothStatus};

#[derive(Default)]
pub struct BlueutilController;

impl BlueutilController {
    /// Runs the underlying command with the specified arguments.
    fn run_command<I, S>(&self, args: I) -> Result<Vec<u8>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = Command::new("/opt/homebrew/bin/blueutil")
            .args(args)
            .output()?;

        if !output.status.success() {
            return Err(eyre!("Failed to run `blueutil` command"));
        }

        Ok(output.stdout)
    }
}

impl BluetoothController for BlueutilController {
    fn get_bluetooth_status(&self) -> Result<BluetoothStatus> {
        let output = self.run_command(["--power"])?;
        let output = std::str::from_utf8(&output)?;

        let status = BluetoothStatus::try_from(output.trim())?;

        Ok(status)
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
