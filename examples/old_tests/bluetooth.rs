use anyhow::Result;
use btleplug::api::{
    bleuuid::BleUuid, Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const PERIPHERAL_NAME_MATCH_FILTER: &str = "Arduino";

const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x13012F00_F8C3_4F4A_A8F4_15CD926DA146);

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                // println!(
                //     "Peripheral {:?} is connected: {:?}",
                //     &local_name, is_connected
                // );
                // Check if it's the peripheral we want.
                if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
                    println!("Found matching peripheral {:?}...", &local_name);
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            eprintln!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    let is_connected = peripheral.is_connected().await?;
                    println!(
                        "Now connected ({:?}) to peripheral {:?}.",
                        is_connected, &local_name
                    );
                    if is_connected {
                        println!("Discover peripheral {:?} services...", local_name);
                        peripheral.discover_services().await?;
                        for characteristic in peripheral.characteristics() {
                            println!("Checking characteristic {:?}", characteristic);
                            // Subscribe to notifications from the characteristic with the selected
                            // UUID.
                            if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
                            // && characteristic.properties.contains(CharPropFlags::NOTIFY)
                            {
                                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await?;
                                // Print the first 4 notifications received.
                                let mut notification_stream = peripheral.notifications().await?;
                                // Process while the BLE connection is not broken or stopped.
                                while let Some(data) = notification_stream.next().await {
                                    // this prints the device name, uuid of the service, and the value retrieved
                                    // println!(
                                    //     "Received data from {:?} [{:?}]: {:?}",
                                    //     local_name, data.uuid, data.value
                                    // );
                                    let crank_value = data.value;
                                    println!("{:?}", crank_value.iter().next().unwrap_or(&0));
                                }
                            }
                        }
                        println!("Disconnecting from peripheral {:?}...", local_name);
                        peripheral.disconnect().await?;
                    }
                } else {
                    // println!("Skipping unknown peripheral {:?}", peripheral);
                }
            }
        }
    }

    Ok(())
}

async fn find_arduino(central: &Adapter, name: String) -> Option<Peripheral> {
    for peripheral in central.peripherals().await.unwrap() {
        if peripheral
            .properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .unwrap_or("Unknown Device".to_string())
            == name
        {
            return Some(peripheral);
        }
    }
    None
}
