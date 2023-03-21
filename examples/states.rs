use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{FontId, RichText};
use nokhwa::query as nquery;
use nokhwa::utils::ApiBackend;
use uhdrtz::prelude::*;

/// TODO:
///    - Have a system run @ OnEnter(RunningStates::Setup) that gets the cameras and their properties as well as the bluetooth devices around (Arduino/RotaryArduino).
///    - run @ OnExit(RunningStates::Setup) a way to write the selected config to a file?

#[derive(Resource, Default)]
struct SelectedCamera(Option<String>);

#[derive(Resource, Default)]
struct CaptureDevices(HashMap<String, u32>);

#[derive(Resource, Default)]
struct Resolution(String);
const RESOLUTIONS: [&'static str; 3] = ["4k30", "1080p60", "1440p60(4:3)"];

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
        .insert_resource(SelectedCamera(Some("Other Device".to_owned())))
        .insert_resource(CaptureDevices::default())
        .insert_resource(Resolution(RESOLUTIONS[0].to_owned()))
        .add_plugin(EguiPlugin)
        .add_state::<RunningStates>()
        .add_system(setup_menu.in_set(OnUpdate(RunningStates::Setup)))
        .add_system(get_cameras.in_schedule(OnEnter(RunningStates::Setup)))
        .add_system(cleanup_menu.in_schedule(OnExit(RunningStates::Setup)))
        .run();
}

fn setup_menu(
    mut ctx: EguiContexts,
    mut selected: ResMut<SelectedCamera>,
    cameras: Res<CaptureDevices>,
    mut quality: ResMut<Resolution>,
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
                    // this makes a new item for each camera that was found
                    for each in RESOLUTIONS {
                        ui.selectable_value(&mut quality.0, each.to_string(), each);
                    }
                });
                ui.end_row();
            });
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
