use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSet, EguiSettings};
use uhdrtz::prelude::*;

/// TODO:
///    - Have a system run @ OnEnter(RunningStates::Setup) that gets the cameras and their properties as well as the bluetooth devices around (Arduino/RotaryArduino).
///    - run @ OnExit(RunningStates::Setup) a way to write the selected config to a file?

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
        .add_plugin(EguiPlugin)
        .add_state::<RunningStates>()
        .add_system(setup_menu.in_set(OnUpdate(RunningStates::Setup)))
        .add_system(cleanup_menu.in_schedule(OnExit(RunningStates::Setup)))
        .run();
}

fn setup_menu(mut ctx: EguiContexts) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.label("Welcome to the UHDRTZ Setup!");
    });
}

fn cleanup_menu() {
    info!("Cleaned up!");
}
