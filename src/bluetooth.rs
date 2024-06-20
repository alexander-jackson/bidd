use std::ffi::OsStr;
use std::process::Command;

use color_eyre::eyre::{eyre, Report, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BluetoothStatus {
    Enabled,
    Disabled,
}

impl TryFrom<&str> for BluetoothStatus {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self> {
        let status = match value {
            "0" => BluetoothStatus::Disabled,
            "1" => BluetoothStatus::Enabled,
            _ => return Err(eyre!("invalid Bluetooth status '{value}' provided")),
        };

        Ok(status)
    }
}

#[derive(Default)]
pub struct BluetoothController;

impl BluetoothController {
    /// Runs the underlying command with the specified arguments.
    fn run_command<I, S>(&self, args: I) -> Result<Vec<u8>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = Command::new("blueutil").args(args).output()?;

        if !output.status.success() {
            return Err(eyre!("Failed to run `blueutil` command"));
        }

        Ok(output.stdout)
    }

    pub fn get_bluetooth_status(&self) -> Result<BluetoothStatus> {
        let output = self.run_command(["--power"])?;
        let output = std::str::from_utf8(&output)?;

        let status = BluetoothStatus::try_from(output.trim())?;

        Ok(status)
    }

    pub fn get_connected_devices(&self) -> Result<Vec<String>> {
        let output = self.run_command(["--connected"])?;
        let output = std::str::from_utf8(&output)?;

        let lines = output.lines().map(|line| line.to_owned()).collect();

        Ok(lines)
    }

    pub fn disable_bluetooth(&self) -> Result<()> {
        self.run_command(["--power", "0"])?;

        Ok(())
    }
}
