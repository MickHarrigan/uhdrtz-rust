use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_tokio_tasks::*;
use egui::{FontId, RichText};
use nokhwa::query as nquery;
use nokhwa::utils::ApiBackend;
use uhdrtz::prelude::*;

#[allow(unused_imports)]
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
#[allow(unused_imports)]
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::time::Duration;
use uuid::Uuid;

const PERIPHERAL_NAME_MATCH_FILTER: &str = "Arduino";

const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x13012F00_F8C3_4F4A_A8F4_15CD926DA146);

/// TODO:
///    - Have a system run @ OnEnter(RunningStates::Setup) that gets the cameras and their properties as well as the bluetooth devices around (Arduino/RotaryArduino).
///    - run @ OnExit(RunningStates::Setup) a way to write the selected config to a file?
///    - find the arduino and have its menu item be a spinner that awaits the finding and connecting to the arduino
///    - add a method to set the capped framerate rotation

#[derive(Resource, Default)]
struct SelectedCamera(Option<String>);

#[derive(Resource, Default)]
struct CaptureDevices(HashMap<String, u32>);

#[derive(Resource, Default)]
struct Resolution(String);

const RESOLUTIONS: [&'static str; 3] = ["4k30", "1080p60", "1440p60(4:3)"];

#[derive(Resource, Default)]
struct ArduinoConnected(bool);

#[derive(Resource)]
pub struct RotationInterval(pub i8); // Converted rotation value for use in external modules

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "UHDRTZ Setup".to_string(),
                position: WindowPosition::Centered {
                    0: MonitorSelection::Primary,
                },
                ..default()
            }),
            ..default()
        }))
        // this line below should be replaced with getting the default camera (index 0)
        .add_plugin(TokioTasksPlugin::default())
        .insert_resource(SelectedCamera(Some("Other Device".to_owned())))
        .insert_resource(CaptureDevices::default())
        .insert_resource(Resolution(RESOLUTIONS[0].to_owned()))
        .insert_resource(ArduinoConnected(false))
        .insert_resource(RotationInterval(0))
        .add_plugin(EguiPlugin)
        .add_state::<RunningStates>()
        .add_system(setup_menu.in_set(OnUpdate(RunningStates::Setup)))
        // .add_system(check_space.in_set(OnUpdate(RunningStates::Setup)))
        .add_system(get_cameras.in_schedule(OnEnter(RunningStates::Setup)))
        // .add_system(find_crank_arduino.in_schedule(OnEnter(RunningStates::Setup)))
        .add_system(async_spawner.in_schedule(OnEnter(RunningStates::Setup)))
        .add_system(cleanup_menu.in_schedule(OnExit(RunningStates::Setup)))
        .run();
}

fn check_space(input: Res<Input<KeyCode>>, mut arduino: ResMut<ArduinoConnected>) {
    // if space is pressed, update the arduino resource
    if input.pressed(KeyCode::Space) {
        arduino.0 = !arduino.0;
    }
}

fn setup_menu(
    mut ctx: EguiContexts,
    mut selected: ResMut<SelectedCamera>,
    cameras: Res<CaptureDevices>,
    mut quality: ResMut<Resolution>,
    arduino: Res<ArduinoConnected>,
    rot: Res<RotationInterval>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("UHDRTZ Setup System").font(FontId::proportional(40.0)));
            ui.label(RichText::new("This is where you can choose the different settings of the camera, arduino, resolution, etc.").font(FontId::proportional(20.0)));
        });

        egui::Ui::add_space(ui, 20.0);

        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // this is where the different items are defined
                // start with a `ui.add()`
                ui.add(egui::Label::new("Camera"));
                egui::ComboBox::from_label(
                    "Select the camera that will be used to capture the video feed",
                )
                .selected_text(format!("{}", selected.0.clone().unwrap_or("No Camera".to_string())))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(50.0);
                    // this makes a new item for each camera that was found
                    for each in cameras.0.iter() {
                        ui.selectable_value(&mut selected.0, Some(each.0.to_string()), each.0);
                    }
                });
                // end with a `ui.end_row()`
                ui.end_row();

                // this is for setting to either 4k30 or 1080p60
                ui.add(egui::Label::new("Quality"));
                egui::ComboBox::from_label(
                    "Select the Quality of the video feed, in combined Resolution and Frame Rate"
                ).selected_text(format!("{}", quality.0)).show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(50.0);
                    for each in RESOLUTIONS {
                        ui.selectable_value(&mut quality.0, each.to_string(), each);
                    }
                });
                ui.end_row();

                // this is the device that should be found such that the crank can be used
                ui.add(egui::Label::new("Crank"));
                // create a spinner that updates to a checkmark when arduino = true
                if !arduino.0 {
                    ui.add(egui::widgets::Spinner::new());
                }
                else {
                    ui.add(egui::Label::new("Rotary Arduino Connected!"));
                }
            });

        // only let this be clicked if arduino.0 is true
        if ui.add_enabled(arduino.0,egui::Button::new("Continue")).clicked() {
            println!("Clicked!");
        }
    });
}

fn cleanup_menu() {
    info!("Cleaned up!");
}

fn get_cameras(mut cams: ResMut<CaptureDevices>, mut selected: ResMut<SelectedCamera>) {
    // this is where the query for cameras should occur and then filter out any repeats
    let cameras = nquery(ApiBackend::Auto).unwrap();
    let mut hash: HashMap<String, _> = HashMap::new();
    for camera in cameras.iter() {
        if hash.contains_key(&camera.human_name()) {
            // if the old value in the hash is greater than the new one
            // replace it with the smaller value
            if hash.get(&camera.human_name()).unwrap() > &camera.index().as_index().unwrap() {
                hash.insert(camera.human_name(), camera.index().as_index().unwrap());
            }
        } else {
            hash.insert(camera.human_name(), camera.index().as_index().unwrap());
        }
    }

    // this sets the default selected camera to that of the first thing obtained from the hash
    if hash.len() > 0 {
        selected.0 = Some(hash.iter().nth(0).unwrap().0.to_string());
    } else {
        selected.0 = None;
    }
    // this sets the "list" of cameras to that of the hash (un-ordered list in effect)
    cams.0 = hash;
}

async fn find_crank_arduino(mut ctx: TaskContext) {
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

pub fn async_spawner(rt: Res<TokioTasksRuntime>) {
    rt.spawn_background_task(find_crank_arduino);
}

// pub async fn get_bluetooth_data() {
//     if is_connected {
//         peripheral.discover_services().await.unwrap();
//         for characteristic in peripheral.characteristics() {
//             if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID {
//                 println!("Subscribing to characteristic {:?}", characteristic.uuid);
//                 peripheral.subscribe(&characteristic).await.unwrap();
//                 let mut notification_stream = peripheral.notifications().await.unwrap();
//                 loop {
//                     if let Some(data) = notification_stream.next().await {
//                         ctx.run_on_main_thread(move |ctx| {
//                             if let Some(mut rotation) =
//                                 ctx.world.get_resource_mut::<RotationInterval>()
//                             {
//                                 let val = *data.value.iter().next().unwrap_or(&0);
//                                 #[allow(unused_assignments)]
//                                 let out: i8;
//                                 if val > 128 {
//                                     out = -1 * (255 - val) as i8;
//                                 } else {
//                                     out = val as i8;
//                                 }

//                                 rotation.0 = out;
//                             }
//                         })
//                         .await;
//                     }
//                 }
//             }
//         }
//         println!("Disconnecting from peripheral {:?}...", local_name);
//         peripheral.disconnect().await.unwrap();
//     }
// }
