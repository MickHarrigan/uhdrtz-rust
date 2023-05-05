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
struct SelectedCamera(Option<(String, u32)>);

#[derive(Resource, Default)]
struct CaptureDevices(HashMap<String, u32>);

#[derive(Resource, Default)]
struct Resolution(String);

const RESOLUTIONS: [&'static str; 3] = ["4k30", "1080p60", "1440p60(4:3)"];

#[derive(Resource, Default, PartialEq)]
pub enum Resolutions {
    #[default]
    Fourk,
    TenEighty,
    FourteenFourty,
}

impl std::fmt::Display for Resolutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fourk => write!(f, "{}", RESOLUTIONS[0]),
            Self::TenEighty => write!(f, "{}", RESOLUTIONS[1]),
            Self::FourteenFourty => write!(f, "{}", RESOLUTIONS[2]),
        }
    }
}

#[derive(Resource, Default)]
struct ArduinoConnected(bool);

#[derive(Resource)]
pub struct RotationInterval(pub i8); // Converted rotation value for use in external modules

#[derive(Resource, Debug)]
pub struct Settings {
    pub camera: nokhwa::utils::CameraIndex,
    pub resolution: nokhwa::utils::Resolution,
    pub frame_rate: u32,
    pub arduino_connection: bool,
}

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
        // .add_plugin(WindowPlugin {
        //     primary_window: Some(Window {
        //         title: "UHDRTZ Setup".to_string(),
        //         position: WindowPosition::Centered {
        //             0: MonitorSelection::Primary,
        //         },
        //         ..default()
        //     }),
        //     ..default()
        // })
        // this line below should be replaced with getting the default camera (index 0)
        .add_plugin(TokioTasksPlugin::default())
        .insert_resource(SelectedCamera(Some(("Other Device".to_owned(), u32::MAX))))
        .insert_resource(CaptureDevices::default())
        // .insert_resource(Resolution(RESOLUTIONS[0].to_owned()))
        .insert_resource(Resolutions::default())
        .insert_resource(Settings {
            camera: nokhwa::utils::CameraIndex::default(),
            resolution: nokhwa::utils::Resolution::default(),
            frame_rate: 0,
            arduino_connection: false,
        })
        .insert_resource(ArduinoConnected(false))
        .insert_resource(RotationInterval(0))
        .add_plugin(EguiPlugin)
        .add_state::<RunningStates>()
        .add_system(setup_menu.in_set(OnUpdate(RunningStates::Setup)))
        // .add_system(get_cameras.in_schedule(OnEnter(RunningStates::Setup)))
        .add_system(async_converter_arduino_finder.in_schedule(OnEnter(RunningStates::Setup)))
        // this line is for getting rid of the old window and opening the typical window
        // .add_system(cleanup_menu.in_schedule(OnExit(RunningStates::Setup)))
        // this line below is where the typical UHDRTZ stuff should happen
        .add_system(async_converter_arduino_reader.in_schedule(OnEnter(RunningStates::Running)))
        .run();
}

// fn cleanup_menu(mut windows: Query<Entity, With<Window>>, mut comms: Commands) {
//     // close up the old window then run the typical zoetrope things
//     for entity in windows.iter() {
//         comms.entity(entity).despawn_recursive();
//     }
// }

// fn setup_menu(
//     mut ctx: EguiContexts,
//     mut selected: ResMut<SelectedCamera>,
//     cameras: Res<CaptureDevices>,
//     mut quality: ResMut<Resolutions>,
//     arduino: Res<ArduinoConnected>,
//     mut next_state: ResMut<NextState<RunningStates>>,
//     mut settings: ResMut<Settings>,
// ) {
//     egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
//         ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
//             ui.label(RichText::new("UHDRTZ Setup System").font(FontId::proportional(40.0)));
//             ui.label(RichText::new("This is where you can choose the different settings of the camera, arduino, resolution, etc.").font(FontId::proportional(20.0)));
//         });

//         egui::Ui::add_space(ui, 20.0);

//         egui::Grid::new("my_grid")
//             .num_columns(2)
//             .spacing([40.0, 4.0])
//             .striped(true)
//             .show(ui, |ui| {
//                 // this is where the different items are defined
//                 // start with a `ui.add()`
//                 ui.add(egui::Label::new("Camera"));
//                 egui::ComboBox::from_label(
//                     "Select the camera that will be used to capture the video feed",
//                 )
//                 .selected_text(format!("{}", selected.0.clone().unwrap_or(("No Camera".to_string(), u32::MAX)).0))
//                 .show_ui(ui, |ui| {
//                     ui.style_mut().wrap = Some(false);
//                     ui.set_min_width(50.0);
//                     // this makes a new item for each camera that was found
//                     for (name, ind) in cameras.0.iter() {
//                         ui.selectable_value(&mut selected.0, Some((name.to_string(), *ind)), name);
//                     }
//                 });
//                 // end with a `ui.end_row()`
//                 ui.end_row();

//                 // this is for setting to either 4k30 or 1080p60
//                 ui.add(egui::Label::new("Quality"));
//                 egui::ComboBox::from_label(
//                     "Select the Quality of the video feed, in combined Resolution and Frame Rate"
//                 ).selected_text(format!("{}", *quality)).show_ui(ui, |ui| {
//                     ui.style_mut().wrap = Some(false);
//                     ui.set_min_width(50.0);
//                     // this is all settings for resolutions
//                     ui.selectable_value(&mut *quality, Resolutions::Fourk, RESOLUTIONS[0]);
//                     ui.selectable_value(&mut *quality, Resolutions::TenEighty, RESOLUTIONS[1]);
//                     ui.selectable_value(&mut *quality, Resolutions::FourteenFourty, RESOLUTIONS[2]);
//                 });
//                 ui.end_row();

//                 // this is the device that should be found such that the crank can be used
//                 ui.add(egui::Label::new("Crank"));
//                 // create a spinner that updates to a checkmark when arduino = true
//                 if !arduino.0 {
//                     ui.add(egui::widgets::Spinner::new());
//                 }
//                 else {
//                     ui.add(egui::Label::new("Rotary Arduino Connected!"));
//                 }
//             });

//         // this is where the settings are converted to nokhwa settings
//         if ui.add_enabled(arduino.0, egui::Button::new("Continue")).clicked() {
//             settings.camera = nokhwa::utils::CameraIndex::Index(selected.0.clone().unwrap().1);
//             (settings.resolution, settings.frame_rate) = match *quality {
//                 Resolutions::Fourk => (nokhwa::utils::Resolution::new(3840, 2160), 30),
//                 Resolutions::TenEighty => (nokhwa::utils::Resolution::new(1920, 1080), 60),
//                 Resolutions::FourteenFourty => (nokhwa::utils::Resolution::new(1920, 1440), 60),
//             };
//             settings.arduino_connection = arduino.0;
//             next_state.set(RunningStates::Running);
//         }
//     });
// }

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
        // selected.0 = Some(hash.iter().nth(0).unwrap().0.to_string());
        let (name, ind) = hash.iter().nth(0).unwrap();
        selected.0 = Some((name.clone().to_string(), *ind));
    } else {
        selected.0 = None;
    }
    // this sets the "list" of cameras to that of the hash (un-ordered list in effect)
    cams.0 = hash;
}

// async fn find_crank_arduino(mut ctx: TaskContext) {
//     let manager = Manager::new().await.unwrap();
//     let adapter_list = manager.adapters().await.unwrap();
//     if adapter_list.is_empty() {
//         error!("No Bluetooth adapters found");
//     }

//     for adapter in adapter_list.iter() {
//         info!("Starting scan...");
//         adapter
//             .start_scan(ScanFilter::default())
//             .await
//             .expect("Can't scan BLE adapter for connected devices...");
//         tokio::time::sleep(Duration::from_secs(2)).await;
//         let peripherals = adapter.peripherals().await.unwrap();

//         if peripherals.is_empty() {
//             error!("->>> BLE peripheral devices were not found, sorry. Exiting...");
//         } else {
//             // All peripheral devices in range.
//             for peripheral in peripherals.iter() {
//                 let properties = peripheral.properties().await.unwrap();
//                 let is_connected = peripheral.is_connected().await.unwrap();
//                 let local_name = properties
//                     .unwrap()
//                     .local_name
//                     .unwrap_or(String::from("(peripheral name unknown)"));
//                 // Check if it's the peripheral we want.
//                 if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
//                     info!("Found matching peripheral {:?}...", &local_name);
//                     if !is_connected {
//                         // Connect if we aren't already connected.
//                         if let Err(err) = peripheral.connect().await {
//                             error!("Error connecting to peripheral, skipping: {}", err);
//                             continue;
//                         }
//                     }
//                     let is_connected = peripheral.is_connected().await.unwrap();
//                     // set the ArduinoConnected to true here
//                     ctx.run_on_main_thread(move |ctx| {
//                         if let Some(mut arduino_connection) =
//                             ctx.world.get_resource_mut::<ArduinoConnected>()
//                         {
//                             arduino_connection.0 = is_connected;
//                         }
//                     })
//                     .await;
//                 }
//             }
//         }
//     }
// }

// pub fn async_converter_arduino_finder(rt: Res<TokioTasksRuntime>) {
//     rt.spawn_background_task(find_crank_arduino);
// }

// pub fn async_converter_arduino_reader(rt: Res<TokioTasksRuntime>) {
//     rt.spawn_background_task(get_bluetooth_data);
// }

// pub async fn get_bluetooth_data(mut ctx: TaskContext) {
//     // absolutely awful lineup of applied functions
//     // this is a breakdown of getting the first adapter from the manager and then a vector of the peripherals from that
//     let peripherals = Manager::new()
//         .await
//         .unwrap()
//         .adapters()
//         .await
//         .unwrap()
//         .first()
//         .unwrap()
//         .peripherals()
//         .await
//         .unwrap();
//     for peripheral in peripherals.iter() {
//         let is_connected = peripheral.is_connected().await.unwrap();

//         if is_connected {
//             peripheral.discover_services().await.unwrap();
//             for characteristic in peripheral.characteristics() {
//                 if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID {
//                     info!("Subscribing to characteristic {:?}", characteristic.uuid);
//                     peripheral.subscribe(&characteristic).await.unwrap();
//                     let mut notification_stream = peripheral.notifications().await.unwrap();
//                     loop {
//                         if let Some(data) = notification_stream.next().await {
//                             ctx.run_on_main_thread(move |ctx| {
//                                 if let Some(mut rotation) =
//                                     ctx.world.get_resource_mut::<RotationInterval>()
//                                 {
//                                     let val = *data.value.iter().next().unwrap_or(&0);
//                                     #[allow(unused_assignments)]
//                                     let out: i8;
//                                     if val > 128 {
//                                         out = -1 * (255 - val) as i8;
//                                     } else {
//                                         out = val as i8;
//                                     }

//                                     rotation.0 = out;
//                                 }
//                             })
//                             .await;
//                         }
//                     }
//                 }
//             }
//             peripheral.disconnect().await.unwrap();
//         }
//     }
// }
