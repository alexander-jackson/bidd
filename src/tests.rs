use std::cell::Cell;

use color_eyre::eyre::Result;

use crate::bluetooth::{BluetoothController, BluetoothStatus};
use crate::LifecycleHandler;

struct MockBluetoothController {
    status: Cell<BluetoothStatus>,
    connected_devices: Vec<String>,
}

impl BluetoothController for MockBluetoothController {
    fn get_bluetooth_status(&self) -> Result<BluetoothStatus> {
        Ok(self.status.get())
    }

    fn get_connected_devices(&self) -> Result<Vec<String>> {
        Ok(self.connected_devices.clone())
    }

    fn disable_bluetooth(&self) -> Result<()> {
        self.status.set(BluetoothStatus::Disabled);

        Ok(())
    }
}

#[test]
fn disables_bluetooth_if_headphones_not_connected() -> Result<()> {
    let controller = MockBluetoothController {
        status: Cell::new(BluetoothStatus::Enabled),
        // no devices are connected
        connected_devices: Vec::new(),
    };

    let handler = LifecycleHandler { controller };

    handler.run()?;

    assert_eq!(handler.controller.status.get(), BluetoothStatus::Disabled);

    Ok(())
}

#[test]
fn bluetooth_is_not_disabled_if_headphones_are_connected() -> Result<()> {
    let controller = MockBluetoothController {
        status: Cell::new(BluetoothStatus::Enabled),
        // headphones are connected
        connected_devices: vec![String::from(
            r#"address: 94-db-56-d9-15-97, connected (master, 0 dBm), not favourite, paired, name: "WH-1000XM4", recent access date: 2024-06-21 09:13:29 +0000"#,
        )],
    };

    let handler = LifecycleHandler { controller };

    handler.run()?;

    assert_eq!(handler.controller.status.get(), BluetoothStatus::Enabled);

    Ok(())
}
