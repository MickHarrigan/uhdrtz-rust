use bevy::prelude::*;
use bevy_tokio_tasks::*;
use futures::stream::StreamExt;
use std::time::Duration;

#[allow(unused_imports)]
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
#[allow(unused_imports)]
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;

// constants
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x13012F00_F8C3_4F4A_A8F4_15CD926DA146);
const PERIPHERAL_NAME_MATCH_FILTER: &str = "Arduino";

// resources
#[derive(Resource)]
pub struct RotationInterval(pub i8); // Converted rotation value for use in external modules

#[derive(Resource, Default)]
pub struct ArduinoConnected(pub bool);

// systems
pub fn async_converter_arduino_reader(rt: Res<TokioTasksRuntime>) {
    rt.spawn_background_task(get_bluetooth_data);
}

pub fn async_converter_arduino_finder(rt: Res<TokioTasksRuntime>) {
    rt.spawn_background_task(find_crank_arduino);
}

pub async fn get_bluetooth_data(mut ctx: TaskContext) {
    // absolutely awful lineup of applied functions
    // this is a breakdown of getting the first adapter from the manager and then a vector of the peripherals from that
    let peripherals = Manager::new()
        .await
        .unwrap()
        .adapters()
        .await
        .unwrap()
        .first()
        .unwrap()
        .peripherals()
        .await
        .unwrap();
    for peripheral in peripherals.iter() {
        let is_connected = peripheral.is_connected().await.unwrap();

        if is_connected {
            peripheral.discover_services().await.unwrap();
            for characteristic in peripheral.characteristics() {
                if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID {
                    info!("Subscribing to characteristic {:?}", characteristic.uuid);
                    peripheral.subscribe(&characteristic).await.unwrap();
                    let mut notification_stream = peripheral.notifications().await.unwrap();
                    loop {
                        if let Some(data) = notification_stream.next().await {
                            ctx.run_on_main_thread(move |ctx| {
                                if let Some(mut rotation) =
                                    ctx.world.get_resource_mut::<RotationInterval>()
                                {
                                    let val = *data.value.iter().next().unwrap_or(&0);
                                    #[allow(unused_assignments)]
                                    let out: i8;
                                    if val > 128 {
                                        out = -1 * (255 - val) as i8;
                                    } else {
                                        out = val as i8;
                                    }

                                    rotation.0 = out;
                                }
                            })
                            .await;
                        }
                    }
                }
            }
            peripheral.disconnect().await.unwrap();
        }
    }
}

pub async fn find_crank_arduino(mut ctx: TaskContext) {
    let manager = Manager::new().await.unwrap();
    let adapter_list = manager.adapters().await.unwrap();
    if adapter_list.is_empty() {
        error!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        info!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        tokio::time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await.unwrap();

        if peripherals.is_empty() {
            error!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await.unwrap();
                let is_connected = peripheral.is_connected().await.unwrap();
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                // Check if it's the peripheral we want.
                if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
                    info!("Found matching peripheral {:?}...", &local_name);
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            error!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    let is_connected = peripheral.is_connected().await.unwrap();
                    // set the ArduinoConnected to true here
                    ctx.run_on_main_thread(move |ctx| {
                        if let Some(mut arduino_connection) =
                            ctx.world.get_resource_mut::<ArduinoConnected>()
                        {
                            arduino_connection.0 = is_connected;
                        }
                    })
                    .await;
                }
            }
        }
    }
}
