use anyhow::Result;
use btleplug::api::{
    bleuuid::BleUuid, Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let manager = Manager::new().await?;
    let adapter = get_central(&manager).await;

    let mut events = adapter.events().await?;

    adapter.start_scan(ScanFilter::default()).await?;

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                println!("Device Discovered: {:?}", id);
            }
            CentralEvent::DeviceConnected(id) => {
                println!("Device Connected: {:?}", id);
            }
            CentralEvent::DeviceDisconnected(id) => {
                println!("Device Disconnected: {:?}", id);
            }
            CentralEvent::ManufacturerDataAdvertisement {
                id,
                manufacturer_data,
            } => {
                println!("Man Data: {:?}, {:?}", id, manufacturer_data);
            }
            CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                println!("Service Data: {:?}, {:?}", id, service_data);
            }
            CentralEvent::ServicesAdvertisement { id, services } => {
                let services: Vec<String> =
                    services.into_iter().map(|s| s.to_short_string()).collect();
                println!("Man Data: {:?}, {:?}", id, services);
            }
            _ => {}
        }
    }

    Ok(())
}

async fn find_arduino(central: &Adapter, mac: String) -> Option<Peripheral> {
    // mac must be in the form of XX:XX:XX:XX:XX:XX where each XX is a pair of hex digits
    // THE COLONS ARE REQUIRED IN MAC
    for peripheral in central.peripherals().await.unwrap() {
        if peripheral
            .properties()
            .await
            .unwrap()
            .unwrap()
            .address
            .to_string()
            == mac
        {
            return Some(peripheral);
        }
    }
    None
}
