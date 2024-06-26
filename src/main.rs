use std::fs::OpenOptions;
use std::path::Path;
use std::time::Duration;

use blueutil::BINARY_LOCATION;
use color_eyre::eyre::{eyre, Result};
use color_eyre::Section;
use tera::{Context, Tera};

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

fn install_launchd_configuration() -> Result<()> {
    // Check that `blueutil` actually exists
    if !Path::new(BINARY_LOCATION).exists() {
        return Err(eyre!("{BINARY_LOCATION} does not exist")
            .suggestion("you can run `brew install blueutil` to install it"));
    }

    let template = include_str!("../resources/bidd.plist");
    let template_name = "plist";

    let mut tera = Tera::default();
    tera.add_raw_template(template_name, &template)?;

    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Failed to get home directory"))?;
    let binary_path = home_dir.join(".cargo").join("bin").join("bidd");

    let config_path = home_dir
        .join("Library")
        .join("LaunchAgents")
        .join("bidd.plist");

    let config_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config_path)?;

    let mut context = Context::new();
    context.insert("binary_path", &binary_path);

    tera.render_to(template_name, &context, config_file)?;

    tracing::info!(
        ?binary_path,
        ?config_path,
        "Installed `launchd` configuration, you may need to allow Bluetooth access shortly"
    );

    Ok(())
}

fn poll_for_changes() -> Result<()> {
    let handler: LifecycleHandler<BlueutilController> = LifecycleHandler::default();

    loop {
        handler.run()?;

        // Wait for a while to check again
        std::thread::sleep(Duration::from_secs(60));
    }
}

fn main() -> Result<()> {
    setup()?;

    match std::env::args().skip(1).next() {
        Some(value) if value == "setup" => install_launchd_configuration()?,
        _ => poll_for_changes()?,
    };

    Ok(())
}

#[cfg(test)]
mod tests;
