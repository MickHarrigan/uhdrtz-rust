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
struct SelectedCamera(String);

#[derive(Resource, Default)]
struct CaptureDevices(HashMap<String, u32>);

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
        .insert_resource(SelectedCamera("Other Device".to_owned()))
        .insert_resource(CaptureDevices::default())
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
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Large").font(FontId::proportional(40.0)));
        });

        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // this is where the different items are defined
                // start with a `ui.add()`
                ui.add(egui::Label::new("Camera"));
                egui::ComboBox::from_label("Select one!")
                    .selected_text(format!("{}", selected.0))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(50.0);
                        // this makes a new item for each camera that was found
                        for each in cameras.0.iter() {
                            ui.selectable_value(&mut selected.0, each.0.to_string(), each.0);
                        }
                    });
                // end with a `ui.end_row()`
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
    selected.0 = hash.iter().nth(0).unwrap().0.to_string();
    // this sets the "list" of cameras to that of the hash (un-ordered list in effect)
    cams.0 = hash;
}
