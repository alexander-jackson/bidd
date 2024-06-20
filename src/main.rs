use std::process::Command;

use color_eyre::eyre::Result;

const HEADPHONE_NAME: &'static str = "WH-1000XM4";

fn setup() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().with_ansi(true).init();

    Ok(())
}

fn main() -> Result<()> {
    setup()?;

    if !are_headphones_on(HEADPHONE_NAME)? {
        Command::new("blueutil").args(["--power", "0"]).spawn()?;

        tracing::info!("Headphones are not on, disabled Bluetooth");
    }

    Ok(())
}

fn are_headphones_on(name: &str) -> Result<bool> {
    let output = Command::new("blueutil").args(["--connected"]).output()?;
    let stdout = std::str::from_utf8(&output.stdout)?;

    Ok(stdout.lines().any(|line| line.contains(name)))
}
