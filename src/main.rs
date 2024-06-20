use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use color_eyre::eyre::{eyre, Result};
use futures_util::StreamExt;

const HEADPHONE_NAME: &'static str = "WH-1000XM4";
const DEVICE_IDENTIFIERS: &[&'static str] = &[
    // "ff3dd8eb-e4b9-b66f-7d04-0e57e7a30766",
    "22a6763b-71bb-27fb-61f0-ac0fa7fd7ab1",
];

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
        let id = match &event {
            CentralEvent::DeviceDiscovered(id)
            | CentralEvent::DeviceUpdated(id)
            | CentralEvent::DeviceConnected(id)
            | CentralEvent::DeviceDisconnected(id)
            | CentralEvent::ManufacturerDataAdvertisement { id, .. }
            | CentralEvent::ServiceDataAdvertisement { id, .. }
            | CentralEvent::ServicesAdvertisement { id, .. } => id,
        };

        if DEVICE_IDENTIFIERS.contains(&id.to_string().as_str()) {
            tracing::info!(?event, "Got an event for a known device");
        }

        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral = central_adapter.peripheral(&id).await?;

                if peripheral
                    .properties()
                    .await?
                    .and_then(|properties| properties.local_name)
                    .is_some_and(|value| value.contains(name))
                {
                    tracing::info!(%id, properties = ?peripheral.properties().await?, "Headphones have been discovered");
                }
            }
            CentralEvent::DeviceDisconnected(id) => {
                tracing::info!(%id, "Device has been disconnected");
            }
            _ => (),
        }
    }

    Ok(false)
}
