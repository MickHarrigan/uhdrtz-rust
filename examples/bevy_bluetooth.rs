use bevy::{core_pipeline::clear_color, prelude::*};
use bevy_tokio_tasks::*;
use futures::stream::StreamExt;
use std::time::Duration;

use btleplug::api::{
    bleuuid::BleUuid, Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;

const PERIPHERAL_NAME_MATCH_FILTER: &str = "Arduino";

const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x13012F00_F8C3_4F4A_A8F4_15CD926DA146);

static COLORS: [Color; 5] = [
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::WHITE,
    Color::BLACK,
];

#[derive(Resource)]
struct ZoetropeRotation(i8);

// Notes:  Next up is to make this work with changing a resource that is user defined.
// This resource will be the "rotation angle" that is desired for the image.
// After setting this the next step is to access that Resource every frame and update the output based on that.

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TokioTasksPlugin::default())
        .insert_resource(ZoetropeRotation(0))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(demo)
        .run();
}

fn demo(rt: ResMut<TokioTasksRuntime>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // rt.spawn_background_task(update_colors);
    rt.spawn_background_task(update_rotation);
}

async fn update_colors(mut ctx: TaskContext) {
    let mut color_index = 0;
    loop {
        ctx.run_on_main_thread(move |ctx| {
            if let Some(mut clear_color) = ctx.world.get_resource_mut::<ClearColor>() {
                clear_color.0 = COLORS[color_index];
                println!("Changed color to {:?}", clear_color.0);
            }
        })
        .await;
        color_index = (color_index + 1) % COLORS.len();
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

async fn update_rotation(mut ctx: TaskContext) {
    // this should update the Res<ArduinoOutput>

    // first part should be run only once, while the sending part should be done constantly
    let manager = Manager::new().await.unwrap();
    let adapter_list = manager.adapters().await.unwrap();
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        tokio::time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await.unwrap();

        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await.unwrap();
                let is_connected = peripheral.is_connected().await.unwrap();
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
                    let is_connected = peripheral.is_connected().await.unwrap();
                    println!(
                        "Now connected ({:?}) to peripheral {:?}.",
                        is_connected, &local_name
                    );
                    if is_connected {
                        println!("Discover peripheral {:?} services...", local_name);
                        peripheral.discover_services().await.unwrap();
                        for characteristic in peripheral.characteristics() {
                            println!("Checking characteristic {:?}", characteristic);
                            // Subscribe to notifications from the characteristic with the selected
                            // UUID.
                            if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
                            // && characteristic.properties.contains(CharPropFlags::NOTIFY)
                            {
                                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await.unwrap();
                                // Print the first 4 notifications received.
                                // Process while the BLE connection is not broken or stopped.
                                let mut notification_stream =
                                    peripheral.notifications().await.unwrap();
                                loop {
                                    if let Some(data) = notification_stream.next().await {
                                        ctx.run_on_main_thread(move |ctx| {
                                            if let Some(mut rotation) =
                                                ctx.world.get_resource_mut::<ZoetropeRotation>()
                                            {
                                                let val = *data.value.iter().next().unwrap_or(&0); // change this to fit the correct rotation values
                                                let mut out: i8 = 0;
                                                if val > 128 {
                                                    out = -1 * (255 - val) as i8;
                                                } else {
                                                    out = val as i8;
                                                }

                                                // if val > MAX {
                                                //     val = MAX;
                                                // } else if val < -1 * MAX {
                                                //     val = -1 * MAX;
                                                // }

                                                rotation.0 = out;
                                                println!(
                                                    "Changed the rotation to {:?}",
                                                    rotation.0
                                                );
                                            }
                                        })
                                        .await;
                                    }
                                }
                            }
                        }
                        println!("Disconnecting from peripheral {:?}...", local_name);
                        peripheral.disconnect().await.unwrap();
                    }
                }
            }
        }
    }
}
