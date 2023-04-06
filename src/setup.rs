use crate::bluetooth::ArduinoConnected;
use crate::audio::Song;
use crate::camera::hash_available_cameras;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_egui::{egui, EguiContexts};
use egui::{FontId, RichText};

#[allow(unused_imports)]
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
#[allow(unused_imports)]
use btleplug::platform::{Adapter, Manager, Peripheral};
// this is going to be where the states will be initialized and where they will control the outcome of the system

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

#[derive(Resource, Debug)]
pub struct Settings {
    pub camera: nokhwa::utils::CameraIndex,
    pub resolution: nokhwa::utils::Resolution,
    pub frame_rate: u32,
    pub arduino_connection: bool,
    pub song: Option<String>,
}
// States that the system can be in
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum RunningStates {
    #[default]
    Setup,
    Running,
    Loading,
    Standby, // this one is more for the fact that there may be some state that I have no idea of that is just sitting and doing nothing
}

pub fn cleanup_menu(mut windows: Query<&mut Window>) {
    // close up the old window then run the typical zoetrope things
    for mut window in &mut windows {
        window.mode = WindowMode::BorderlessFullscreen;
        window.present_mode = PresentMode::AutoVsync;
        window.title = "UHDRTZ".to_string();
    }
}

pub fn setup_menu(
    mut ctx: EguiContexts,
    mut quality: ResMut<Resolutions>,
    mut song: ResMut<Song>,
    arduino: Res<ArduinoConnected>,
    mut next_state: ResMut<NextState<RunningStates>>,
    mut settings: ResMut<Settings>,
) {
    let (mut selected, cameras) = hash_available_cameras();
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("UHDRTZ Setup System").font(FontId::proportional(40.0)));
            ui.label(RichText::new("This is where you can choose the different settings of the camera, arduino, resolution, etc.").font(FontId::proportional(20.0)));
        });

        egui::Ui::add_space(ui, 20.0);

        egui::Grid::new("my_grid")
            .num_columns(3)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // this is where the different items are defined
                // start with a `ui.add()`
                ui.add(egui::Label::new("Camera"));
                egui::ComboBox::from_label(
                    "Select the camera that will be used to capture the video feed",
                )
                .selected_text(format!("{}", selected.clone().unwrap_or(("No Camera".to_string(), u32::MAX)).0))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(50.0);
                    // this makes a new item for each camera that was found
                    for (name, ind) in cameras.iter() {
                        ui.selectable_value(&mut selected, Some((name.to_string(), *ind)), name);
                    }
                });
                // end with a `ui.end_row()`
                ui.end_row();

                // this is for setting to either 4k30 or 1080p60
                ui.add(egui::Label::new("Quality"));
                egui::ComboBox::from_label(
                    "Select the Quality of the video feed, in combined Resolution and Frame Rate"
                ).selected_text(format!("{}", *quality)).show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(50.0);
                    // this is all settings for resolutions
                    ui.selectable_value(&mut *quality, Resolutions::Fourk, RESOLUTIONS[0]);
                    ui.selectable_value(&mut *quality, Resolutions::TenEighty, RESOLUTIONS[1]);
                    ui.selectable_value(&mut *quality, Resolutions::FourteenFourty, RESOLUTIONS[2]);
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
                ui.end_row();

                // music changer
                ui.add(egui::Label::new("Audio"));
                egui::ComboBox::from_label("Select the song to play during the animation. Must be an mp3.").selected_text(format!("{}", song.0)).show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(50.0);

                    let paths = std::fs::read_dir("./assets/audio").unwrap();

                    for path in paths {
                        let file = path.unwrap().file_name().into_string().unwrap();
                        ui.selectable_value(&mut song.0, file.clone(), file);
                    }
                    // extra listing for no music
                    ui.selectable_value(&mut song.0,"None".into(), "None");
                });
                // make a button to open the audio location
                if ui.add(egui::Button::new("Open Audio Location")).clicked() {
                    // open the location for audio in the default file browser
                    std::process::Command::new("xdg-open").arg("./assets/audio").spawn().unwrap();
                }
                ui.end_row();

                // location for setting the "slices" of the art?
                // TODO: Ask dyer about this section and if it should be included in a certain manner
                
            });

        // this is where the settings are converted to nokhwa settings
        if ui.add_enabled(arduino.0, egui::Button::new("Continue")).clicked() {
            settings.camera = nokhwa::utils::CameraIndex::Index(selected.clone().unwrap().1);
            (settings.resolution, settings.frame_rate) = match *quality {
                Resolutions::Fourk => (nokhwa::utils::Resolution::new(3840, 2160), 30),
                Resolutions::TenEighty => (nokhwa::utils::Resolution::new(1920, 1080), 60),
                Resolutions::FourteenFourty => (nokhwa::utils::Resolution::new(1920, 1440), 60),
            };
            settings.arduino_connection = arduino.0;
            // update the song things so that its confirmed a correct song
            settings.song = match song.0.as_str() {
                "None" => None,
                a => Some(a.to_string()),
            };

            next_state.set(RunningStates::Running);
        }
    });
}