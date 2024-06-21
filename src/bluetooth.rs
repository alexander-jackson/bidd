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

pub trait BluetoothController {
    fn get_bluetooth_status(&self) -> Result<BluetoothStatus>;
    fn get_connected_devices(&self) -> Result<Vec<String>>;
    fn disable_bluetooth(&self) -> Result<()>;
}
