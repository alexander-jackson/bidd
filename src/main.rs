use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use color_eyre::eyre::{eyre, Result};
use futures_util::StreamExt;

const HEADPHONE_NAME: &'static str = "WH-1000XM4";

fn setup() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().with_ansi(true).init();

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup()?;

    let headphones_on = are_headphones_on(HEADPHONE_NAME).await?;
    tracing::info!(%headphones_on, "Checked the system state");

    Ok(())
}

async fn are_headphones_on(name: &str) -> Result<bool> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;

    let central_adapter = adapters
        .first()
        .ok_or_else(|| eyre!("Failed to find any adapters"))?;

    central_adapter.start_scan(ScanFilter::default()).await?;

    // Subscribe to events
    let mut events = central_adapter.events().await?;

    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            let peripheral = central_adapter.peripheral(&id).await?;

            tracing::info!(%id, "Discovered a new device");

            if peripheral
                .properties()
                .await?
                .and_then(|properties| properties.local_name)
                .is_some_and(|value| value.contains(name))
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
